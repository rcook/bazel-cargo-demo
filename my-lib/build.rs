use anyhow::{bail, Result};
use path_absolutize::Absolutize;
use std::env::{current_dir, var, VarError};
use std::ffi::OsStr;
use std::fs::{metadata, read_dir};
use std::path::{Path, PathBuf};

const RUNNING_IN_BAZEL_ENV: &str = "RUNNING_IN_BAZEL";
const WORKSPACE_NAME: &str = "my-app";
const PROTO_DIR: &str = "proto";

fn is_running_in_bazel() -> Result<bool> {
    Ok(match var(RUNNING_IN_BAZEL_ENV) {
        Ok(s) => s == "1" || s.to_lowercase() == "true",
        Err(VarError::NotPresent) => false,
        Err(e) => bail!(e),
    })
}

fn get_proto_dir(is_running_in_bazel: bool) -> Result<PathBuf> {
    Ok(if is_running_in_bazel {
        PathBuf::from(format!(
            "../../{}/{}/_virtual_imports/{}",
            WORKSPACE_NAME, PROTO_DIR, PROTO_DIR
        ))
        .absolutize_from(current_dir()?)?
        .to_path_buf()
    } else {
        PathBuf::from(format!("../{}", PROTO_DIR))
            .absolutize_from(current_dir()?)?
            .to_path_buf()
    })
}

fn list_files(start_dir: &Path, ext: Option<&OsStr>) -> Result<Vec<PathBuf>> {
    fn helper(paths: &mut Vec<PathBuf>, start_dir: &Path, ext: &Option<&OsStr>) -> Result<()> {
        for result in read_dir(start_dir)? {
            let entry = result?;
            let p = entry.path().absolutize_from(start_dir)?.to_path_buf();
            let m = metadata(&p)?;
            if m.is_dir() {
                helper(paths, &p, ext)?;
            } else if m.is_file() {
                if ext.is_some() {
                    if p.extension() == *ext {
                        paths.push(p);
                    }
                } else {
                    paths.push(p);
                }
            }
        }
        Ok(())
    }

    assert!(start_dir.is_absolute());
    let mut paths = Vec::new();
    helper(&mut paths, start_dir, &ext)?;
    Ok(paths)
}

fn main() -> Result<()> {
    let is_running_in_bazel = is_running_in_bazel()?;
    let proto_dir = get_proto_dir(is_running_in_bazel)?;
    let protos = list_files(&proto_dir, Some(OsStr::new("proto")))?;
    tonic_build::configure().compile(&protos, &[proto_dir])?;
    Ok(())
}
