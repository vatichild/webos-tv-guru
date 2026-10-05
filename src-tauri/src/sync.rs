//! Folder sync: watch a folder of `.ipk` files and keep the chosen TVs up to date.

use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use notify_debouncer_full::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, RecommendedCache};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use crate::apps::{self, InstallProgress};
use crate::error::{AppError, AppResult};
use crate::ipk::{self, IpkInfo};
use crate::{devices, version};

pub const ENTRIES_EVENT: &str = "sync://entries";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncConfig {
    pub folder: Option<String>,
    pub devices: Vec<String>,
    /// Install missing and outdated apps without asking.
    pub auto_install: bool,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SyncState {
    Checking,
    UpToDate,
    Missing,
    Outdated,
    /// The TV has a newer version than the file.
    TvNewer,
    Installing,
    Failed,
    Offline,
    Invalid,
}

/// One `.ipk` file against one TV.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncEntry {
    pub file: String,
    pub device: String,
    pub package: Option<String>,
    pub version: Option<String>,
    pub installed_version: Option<String>,
    pub state: SyncState,
    pub progress: Option<u8>,
    pub message: Option<String>,
}

pub struct SyncManager {
    app: AppHandle,
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    config: SyncConfig,
    watcher: Option<Debouncer<RecommendedWatcher, RecommendedCache>>,
    entries: Vec<SyncEntry>,
    scanning: bool,
    /// Another scan was asked for while one ran; `Some(true)` if it should install.
    pending: Option<bool>,
}

impl SyncManager {
    pub fn new(app: AppHandle) -> Arc<Self> {
        let manager = Arc::new(Self {
            app,
            inner: Mutex::new(Inner::default()),
        });
        let config = manager.load_config().unwrap_or_default();
        if let Err(e) = manager.apply_config(config) {
            log::warn!("Sync config not applied: {e}");
        }
        manager
    }

    pub fn config(&self) -> SyncConfig {
        self.inner.lock().unwrap().config.clone()
    }

    pub fn entries(&self) -> Vec<SyncEntry> {
        self.inner.lock().unwrap().entries.clone()
    }

    pub fn set_config(self: &Arc<Self>, config: SyncConfig) -> AppResult<()> {
        self.apply_config(config.clone())?;
        let path = self.config_path()?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        fs::write(path, serde_json::to_vec_pretty(&config)?)?;
        self.trigger(false);
        Ok(())
    }

    /// Follow a TV that was renamed (`new` is `Some`) or removed (`None`).
    pub fn device_renamed(self: &Arc<Self>, old: &str, new: Option<&str>) -> AppResult<()> {
        let mut config = self.config();
        if !config.devices.iter().any(|d| d == old) {
            return Ok(());
        }
        config.devices = config
            .devices
            .into_iter()
            .filter_map(|d| if d == old { new.map(String::from) } else { Some(d) })
            .collect();
        self.set_config(config)
    }

    /// Start a scan, installing what's missing when `install` is set or when
    /// auto-install is on. A scan already running picks the request up after.
    pub fn trigger(self: &Arc<Self>, install: bool) {
        {
            let mut inner = self.inner.lock().unwrap();
            if inner.scanning {
                inner.pending = Some(inner.pending.unwrap_or(false) || install);
                return;
            }
            inner.scanning = true;
        }
        let this = Arc::clone(self);
        thread::spawn(move || {
            let mut install = install;
            loop {
                let config = this.config();
                this.scan(&config, install || config.auto_install);
                let mut inner = this.inner.lock().unwrap();
                match inner.pending.take() {
                    Some(next) => install = next,
                    None => {
                        inner.scanning = false;
                        break;
                    }
                }
            }
        });
    }

    fn apply_config(self: &Arc<Self>, config: SyncConfig) -> AppResult<()> {
        let watcher = match &config.folder {
            Some(folder) if !folder.is_empty() => Some(self.watch(Path::new(folder))?),
            _ => None,
        };
        let mut inner = self.inner.lock().unwrap();
        inner.config = config;
        inner.watcher = watcher;
        Ok(())
    }

    fn watch(self: &Arc<Self>, folder: &Path) -> AppResult<Debouncer<RecommendedWatcher, RecommendedCache>> {
        let weak = Arc::downgrade(self);
        let mut debouncer = new_debouncer(
            Duration::from_secs(2),
            None,
            move |result: DebounceEventResult| {
                let Ok(events) = result else { return };
                let touches_ipk = events
                    .iter()
                    .flat_map(|e| e.paths.iter())
                    .any(|p| is_ipk(p));
                if let (true, Some(this)) = (touches_ipk, weak.upgrade()) {
                    this.trigger(false);
                }
            },
        )
        .map_err(|e| AppError::other(format!("Can't watch folder: {e}")))?;
        debouncer
            .watch(folder, RecursiveMode::NonRecursive)
            .map_err(|e| AppError::other(format!("Can't watch {}: {e}", folder.display())))?;
        Ok(debouncer)
    }

    fn scan(&self, config: &SyncConfig, install: bool) {
        let packages = match &config.folder {
            Some(folder) if !folder.is_empty() => read_packages(Path::new(folder)),
            _ => Vec::new(),
        };
        let mut entries = Vec::new();
        for device in &config.devices {
            for (file, _, info) in &packages {
                entries.push(SyncEntry {
                    file: file.clone(),
                    device: device.clone(),
                    package: info.as_ref().ok().map(|i| i.package.clone()),
                    version: info.as_ref().ok().map(|i| i.version.clone()),
                    installed_version: None,
                    state: if info.is_ok() {
                        SyncState::Checking
                    } else {
                        SyncState::Invalid
                    },
                    progress: None,
                    message: info.as_ref().err().map(ToString::to_string),
                });
            }
        }
        self.update(|e| *e = entries);

        for device in &config.devices {
            if let Err(e) = self.sync_device(device, &packages, install) {
                let message = e.to_string();
                self.update(|entries| {
                    for entry in entries.iter_mut().filter(|x| &x.device == device) {
                        if entry.state == SyncState::Checking {
                            entry.state = SyncState::Offline;
                            entry.message = Some(message.clone());
                        }
                    }
                });
            }
        }
    }

    fn sync_device(
        &self,
        device: &str,
        packages: &[(String, PathBuf, AppResult<IpkInfo>)],
        install: bool,
    ) -> AppResult<()> {
        let session = devices::connect(device)?;
        let mut installed = apps::installed_versions(&session)?;

        let state_of = |info: &IpkInfo, installed: &std::collections::HashMap<String, String>| {
            match installed.get(&info.package) {
                None => (SyncState::Missing, None),
                Some(have) => (
                    match version::compare(&info.version, have) {
                        Ordering::Greater => SyncState::Outdated,
                        Ordering::Equal => SyncState::UpToDate,
                        Ordering::Less => SyncState::TvNewer,
                    },
                    Some(have.clone()),
                ),
            }
        };

        self.update(|entries| {
            for (file, _, info) in packages {
                let Ok(info) = info else { continue };
                let (state, have) = state_of(info, &installed);
                if let Some(entry) = find_entry(entries, file, device) {
                    entry.state = state;
                    entry.installed_version = have;
                }
            }
        });

        if !install {
            return Ok(());
        }
        for (file, path, info) in packages {
            let Ok(info) = info else { continue };
            let (state, _) = state_of(info, &installed);
            if !matches!(state, SyncState::Missing | SyncState::Outdated) {
                continue;
            }
            self.set_entry(file, device, |e| {
                e.state = SyncState::Installing;
                e.progress = Some(0);
                e.message = None;
            });
            let result = apps::install(&session, path, |p| self.report(file, device, p));
            match result {
                Ok(_) => {
                    installed.insert(info.package.clone(), info.version.clone());
                    self.set_entry(file, device, |e| {
                        e.state = SyncState::UpToDate;
                        e.installed_version = Some(info.version.clone());
                        e.progress = None;
                        e.message = None;
                    });
                }
                Err(err) => self.set_entry(file, device, |e| {
                    e.state = SyncState::Failed;
                    e.progress = None;
                    e.message = Some(err.to_string());
                }),
            }
        }
        Ok(())
    }

    fn report(&self, file: &str, device: &str, progress: InstallProgress) {
        let (pct, message) = match progress {
            InstallProgress::Uploading { sent, total } if total > 0 => {
                (Some((sent * 100 / total).min(100) as u8), Some("Uploading".to_string()))
            }
            InstallProgress::Uploading { .. } => (None, Some("Uploading".to_string())),
            InstallProgress::Verifying => (Some(100), Some("Verifying".to_string())),
            InstallProgress::Installing { status } => (Some(100), Some(status)),
            InstallProgress::Done { .. } => return,
        };
        let changed = {
            let inner = self.inner.lock().unwrap();
            inner
                .entries
                .iter()
                .find(|e| e.file == file && e.device == device)
                .is_none_or(|e| e.progress != pct || e.message != message)
        };
        if changed {
            self.set_entry(file, device, |e| {
                e.progress = pct;
                e.message = message;
            });
        }
    }

    fn set_entry(&self, file: &str, device: &str, f: impl FnOnce(&mut SyncEntry)) {
        self.update(|entries| {
            if let Some(entry) = find_entry(entries, file, device) {
                f(entry);
            }
        });
    }

    fn update(&self, f: impl FnOnce(&mut Vec<SyncEntry>)) {
        let snapshot = {
            let mut inner = self.inner.lock().unwrap();
            f(&mut inner.entries);
            inner.entries.clone()
        };
        if let Err(e) = self.app.emit(ENTRIES_EVENT, snapshot) {
            log::warn!("Failed to emit sync entries: {e}");
        }
    }

    fn config_path(&self) -> AppResult<PathBuf> {
        Ok(self
            .app
            .path()
            .app_config_dir()
            .map_err(|e| AppError::other(e.to_string()))?
            .join("sync.json"))
    }

    fn load_config(&self) -> AppResult<SyncConfig> {
        Ok(serde_json::from_slice(&fs::read(self.config_path()?)?)?)
    }
}

fn find_entry<'a>(entries: &'a mut [SyncEntry], file: &str, device: &str) -> Option<&'a mut SyncEntry> {
    entries
        .iter_mut()
        .find(|e| e.file == file && e.device == device)
}

fn is_ipk(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("ipk"))
}

/// Every `.ipk` in `folder`, newest version first within a package, so a
/// stale copy of an app never replaces a newer one.
fn read_packages(folder: &Path) -> Vec<(String, PathBuf, AppResult<IpkInfo>)> {
    let Ok(dir) = fs::read_dir(folder) else {
        return Vec::new();
    };
    let mut packages: Vec<_> = dir
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_ipk(p))
        .map(|p| {
            let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
            let info = ipk::read_info(&p);
            (name, p, info)
        })
        .collect();
    packages.sort_by(|(fa, _, a), (fb, _, b)| match (a, b) {
        (Ok(a), Ok(b)) => a
            .package
            .cmp(&b.package)
            .then_with(|| version::compare(&b.version, &a.version))
            .then_with(|| fa.cmp(fb)),
        (Ok(_), Err(_)) => Ordering::Less,
        (Err(_), Ok(_)) => Ordering::Greater,
        (Err(_), Err(_)) => fa.cmp(fb),
    });
    packages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipk::tests::build_ipk;

    #[test]
    fn newest_version_of_a_package_comes_first() {
        let dir = std::env::temp_dir().join(format!("guru-sync-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("old.ipk"), build_ipk("Package: a.b\nVersion: 1.2.0\n")).unwrap();
        fs::write(dir.join("new.ipk"), build_ipk("Package: a.b\nVersion: 1.10.0\n")).unwrap();
        fs::write(dir.join("notes.txt"), "x").unwrap();

        let packages = read_packages(&dir);
        let names: Vec<_> = packages.iter().map(|(n, _, _)| n.as_str()).collect();
        assert_eq!(names, ["new.ipk", "old.ipk"]);

        fs::remove_dir_all(&dir).ok();
    }
}
