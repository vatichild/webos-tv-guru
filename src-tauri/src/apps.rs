//! App management on a TV: list, install, remove, launch.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use ares_connection_lib::luna::{Luna, LunaEmptyPayload, Message};
use ares_connection_lib::session::DeviceSession;
use ares_connection_lib::transfer::FileTransfer;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::error::{AppError, AppResult};

const TEMP_DIR: &str = "/media/developer/temp";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub id: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub title: String,
    #[serde(default, rename(deserialize = "type"))]
    pub app_type: String,
    #[serde(default)]
    pub vendor: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
}

#[derive(Deserialize)]
struct ListAppsResponse {
    apps: Vec<InstalledApp>,
}

/// Steps reported while an install runs.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "stage", rename_all = "camelCase")]
pub enum InstallProgress {
    Uploading { sent: u64, total: u64 },
    Verifying,
    Installing { status: String },
    Done { package_id: String },
}

pub fn list_installed(session: &DeviceSession) -> AppResult<Vec<InstalledApp>> {
    let dev: Result<ListAppsResponse, _> = session.call(
        "luna://com.webos.applicationManager/dev/listApps",
        LunaEmptyPayload::default(),
        true,
    );
    let response = match dev {
        Ok(r) => r,
        Err(_) => session.call(
            "luna://com.webos.applicationManager/listApps",
            LunaEmptyPayload::default(),
            false,
        )?,
    };
    Ok(response.apps)
}

/// Installed version per app id.
pub fn installed_versions(session: &DeviceSession) -> AppResult<HashMap<String, String>> {
    Ok(list_installed(session)?
        .into_iter()
        .map(|a| (a.id, a.version))
        .collect())
}

pub fn install(
    session: &DeviceSession,
    package: &Path,
    progress: impl Fn(InstallProgress),
) -> AppResult<String> {
    let checksum = sha256_file(package)?;
    let mut file = File::open(package)?;
    let total = file.metadata()?.len();
    let remote = format!("{TEMP_DIR}/guru_{}.ipk", &checksum[..12]);

    session.mkdir(TEMP_DIR, 0o777)?;
    session.put(&mut file, &remote, |sent| {
        progress(InstallProgress::Uploading {
            sent: sent as u64,
            total,
        });
    })?;

    let result = verify_and_install(session, &remote, &checksum, &progress);
    if let Err(e) = session.rm(&remote) {
        log::warn!("Failed to delete {remote}: {e}");
    }
    let package_id = result?;
    progress(InstallProgress::Done {
        package_id: package_id.clone(),
    });
    Ok(package_id)
}

fn verify_and_install(
    session: &DeviceSession,
    remote: &str,
    checksum: &str,
    progress: &impl Fn(InstallProgress),
) -> AppResult<String> {
    progress(InstallProgress::Verifying);
    if let Some(actual) = session.sha256sum(remote)? {
        if actual != checksum {
            return Err(AppError::Checksum {
                expected: checksum.to_string(),
                actual,
            });
        }
    }
    let subscription = session.subscribe(
        "luna://com.webos.appInstallService/dev/install",
        json!({ "id": "com.ares.defaultName", "ipkUrl": remote, "subscribe": true }),
        true,
    )?;
    wait_for(subscription, "installed", |status| {
        progress(InstallProgress::Installing { status })
    })
}

pub fn remove(session: &DeviceSession, id: &str) -> AppResult<()> {
    let subscription = session.subscribe(
        "luna://com.webos.appInstallService/dev/remove",
        json!({ "id": id, "subscribe": true }),
        true,
    )?;
    wait_for(subscription, "removed", |_| {})?;
    Ok(())
}

pub fn launch(session: &DeviceSession, id: &str) -> AppResult<()> {
    let _: serde_json::Value = session.call(
        "luna://com.webos.applicationManager/launch",
        json!({ "id": id }),
        true,
    )?;
    Ok(())
}

/// Read appinstalld status messages until one is final.
fn wait_for(
    messages: impl IntoIterator<Item = std::io::Result<Message>>,
    done_word: &str,
    on_status: impl Fn(String),
) -> AppResult<String> {
    for message in messages {
        let value: serde_json::Value = message?.deserialize()?;
        match status_of(&value, done_word) {
            Status::Pending(Some(s)) => on_status(s),
            Status::Pending(None) => {}
            Status::Done(id) => return Ok(id),
            Status::Failed { code, reason } => return Err(AppError::Install { code, reason }),
        }
    }
    Err(AppError::other("The TV closed the connection before the operation finished"))
}

#[derive(Debug, PartialEq)]
enum Status {
    Pending(Option<String>),
    Done(String),
    Failed { code: i32, reason: String },
}

fn status_of(value: &serde_json::Value, done_word: &str) -> Status {
    if value.get("returnValue") == Some(&serde_json::Value::Bool(false)) {
        return Status::Failed {
            code: value["errorCode"].as_i64().unwrap_or(0) as i32,
            reason: value["errorText"]
                .as_str()
                .unwrap_or("request rejected")
                .to_string(),
        };
    }
    let details = &value["details"];
    let Some(state) = details["state"].as_str() else {
        return Status::Pending(None);
    };
    let lower = state.to_lowercase();
    if lower.contains("failed") {
        let code = details["errorCode"].as_i64().unwrap_or(0) as i32;
        let reason = details["reason"].as_str().map(String::from).unwrap_or_else(|| {
            if code == -5 {
                "Not enough free space on the TV".into()
            } else {
                "unknown error".into()
            }
        });
        return Status::Failed { code, reason };
    }
    if lower.starts_with("success") || lower.contains(done_word) {
        return Status::Done(details["packageId"].as_str().unwrap_or_default().to_string());
    }
    Status::Pending(Some(state.strip_prefix("installing : ").unwrap_or(state).to_string()))
}

fn sha256_file(path: &Path) -> AppResult<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn installing_is_pending() {
        let v = json!({"details": {"state": "installing : 40%"}});
        assert_eq!(status_of(&v, "installed"), Status::Pending(Some("40%".into())));
    }

    #[test]
    fn installed_is_done() {
        let v = json!({"details": {"state": "installed", "packageId": "a.b"}});
        assert_eq!(status_of(&v, "installed"), Status::Done("a.b".into()));
        let v = json!({"details": {"state": "SUCCESS", "packageId": "a.b"}});
        assert_eq!(status_of(&v, "installed"), Status::Done("a.b".into()));
    }

    #[test]
    fn no_space_is_explained() {
        let v = json!({"details": {"state": "FAILED", "errorCode": -5}});
        assert_eq!(
            status_of(&v, "installed"),
            Status::Failed {
                code: -5,
                reason: "Not enough free space on the TV".into()
            }
        );
    }

    #[test]
    fn rejected_call_fails() {
        let v = json!({"returnValue": false, "errorCode": -1, "errorText": "Unknown method"});
        assert!(matches!(status_of(&v, "installed"), Status::Failed { .. }));
    }

    #[test]
    fn subscribe_ack_is_pending() {
        let v = json!({"returnValue": true, "subscribed": true});
        assert_eq!(status_of(&v, "removed"), Status::Pending(None));
    }
}
