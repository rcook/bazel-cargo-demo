use anyhow::Result;
use path_absolutize::Absolutize;
use std::env::current_dir;
use std::path::PathBuf;

fn main() -> Result<()> {
    let cwd = current_dir()?;
    let proto_dir = PathBuf::from("../my-proto")
        .absolutize_from(cwd)?
        .to_path_buf();

    let protos = vec!["greeter.proto"];

    for proto in protos {
        tonic_build::compile_protos(proto_dir.join(proto))?;
    }

    Ok(())
}
