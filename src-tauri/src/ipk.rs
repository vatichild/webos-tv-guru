//! Reads the package metadata out of an `.ipk` file.
//!
//! An ipk is an `ar` archive holding `debian-binary`, `control.tar.gz` and
//! `data.tar.gz`. The `control` file inside `control.tar.gz` names the package.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::Serialize;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IpkInfo {
    pub package: String,
    pub version: String,
    pub architecture: Option<String>,
}

pub fn read_info(path: &Path) -> AppResult<IpkInfo> {
    read_info_from(BufReader::new(File::open(path)?))
}

pub fn read_info_from<R: Read>(reader: R) -> AppResult<IpkInfo> {
    let mut archive = ar::Archive::new(reader);
    while let Some(entry) = archive.next_entry() {
        let entry = entry.map_err(|e| AppError::Ipk(format!("not an ar archive: {e}")))?;
        let name = String::from_utf8_lossy(entry.header().identifier()).to_string();
        let name = name.trim_end_matches('/');
        if name == "control.tar.gz" {
            return read_control_tar(flate2::read::GzDecoder::new(entry));
        }
        if name == "control.tar" {
            return read_control_tar(entry);
        }
        if name.starts_with("control.tar") {
            return Err(AppError::Ipk(format!("unsupported control archive {name}")));
        }
    }
    Err(AppError::Ipk("no control archive".into()))
}

fn read_control_tar<R: Read>(reader: R) -> AppResult<IpkInfo> {
    let mut tar = tar::Archive::new(reader);
    for file in tar.entries()? {
        let mut file = file?;
        let path = file.path()?.to_string_lossy().to_string();
        if path.trim_start_matches("./") == "control" {
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            return parse_control(&text);
        }
    }
    Err(AppError::Ipk("control file missing".into()))
}

fn parse_control(text: &str) -> AppResult<IpkInfo> {
    let field = |key: &str| {
        text.lines().find_map(|line| {
            let (k, v) = line.split_once(':')?;
            (k.trim().eq_ignore_ascii_case(key)).then(|| v.trim().to_string())
        })
    };
    Ok(IpkInfo {
        package: field("Package").ok_or_else(|| AppError::Ipk("no Package field".into()))?,
        version: field("Version").ok_or_else(|| AppError::Ipk("no Version field".into()))?,
        architecture: field("Architecture"),
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Build an ipk in memory with the given control file.
    pub(crate) fn build_ipk(control: &str) -> Vec<u8> {
        let mut control_tar = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_size(control.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        control_tar
            .append_data(&mut header, "./control", control.as_bytes())
            .unwrap();
        let control_tar = control_tar.into_inner().unwrap();

        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        std::io::Write::write_all(&mut gz, &control_tar).unwrap();
        let control_tgz = gz.finish().unwrap();

        let mut out = ar::Builder::new(Vec::new());
        out.append(&ar::Header::new(b"debian-binary".to_vec(), 4), &b"2.0\n"[..])
            .unwrap();
        out.append(
            &ar::Header::new(b"control.tar.gz".to_vec(), control_tgz.len() as u64),
            &control_tgz[..],
        )
        .unwrap();
        out.into_inner().unwrap()
    }

    #[test]
    fn reads_package_and_version() {
        let ipk = build_ipk(
            "Package: com.example.app\nVersion: 0.7.1\nArchitecture: arm\nDescription: x\n",
        );
        let info = read_info_from(&ipk[..]).unwrap();
        assert_eq!(info.package, "com.example.app");
        assert_eq!(info.version, "0.7.1");
        assert_eq!(info.architecture.as_deref(), Some("arm"));
    }

    #[test]
    fn missing_version_is_an_error() {
        let ipk = build_ipk("Package: a.b\n");
        assert!(read_info_from(&ipk[..]).is_err());
    }

    #[test]
    fn garbage_is_an_error() {
        assert!(read_info_from(&b"not an ipk"[..]).is_err());
    }
}
