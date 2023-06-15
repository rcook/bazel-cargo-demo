use anyhow::{bail, Result};
use path_absolutize::Absolutize;
use std::env::{current_dir, var, VarError};
use std::ffi::OsStr;
use std::fs::{metadata, read_dir};
use std::path::{Path, PathBuf};

const RUNNING_IN_BAZEL_ENV: &str = "RUNNING_IN_BAZEL";
const PROTO_DIR: &str = "proto";
const PROTO_EXT: &str = "proto";

fn get_proto_dir() -> Result<PathBuf> {
    fn is_running_in_bazel() -> Result<bool> {
        Ok(match var(RUNNING_IN_BAZEL_ENV) {
            Ok(s) => s == "1" || s.to_lowercase() == "true",
            Err(VarError::NotPresent) => false,
            Err(e) => bail!(e),
        })
    }

    let proto_dir_rel = if is_running_in_bazel()? {
        format!("../{PROTO_DIR}/_virtual_imports/{PROTO_DIR}")
    } else {
        format!("../{PROTO_DIR}")
    };

    Ok(Path::new(&proto_dir_rel)
        .absolutize_from(current_dir()?)?
        .to_path_buf())
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
    let proto_dir = get_proto_dir()?;
    let protos = list_files(&proto_dir, Some(OsStr::new(PROTO_EXT)))?;
    tonic_build::configure().compile(&protos, &[proto_dir])?;
    Ok(())
}
