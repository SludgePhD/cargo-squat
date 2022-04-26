use std::{
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    process::{exit, Command},
};

use cargo_metadata::MetadataCommand;
use tempfile::tempdir;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(2).collect::<Vec<_>>();
    let package = match &*args {
        [] => {
            let metadata = MetadataCommand::new()
                .exec()
                .map_err(|e| format!("failed to run `cargo metadata`: {e}"))?;
            match metadata.root_package() {
                Some(pkg) => pkg.name.clone(),
                None => {
                    eprintln!("`cargo squat` was invoked on a virtual manifest, invoke it on a real manifest or specify the package name");
                    exit(1);
                }
            }
        }
        [package] => package.clone(),
        _ => {
            eprintln!("usage: cargo squat [<package>]");
            exit(1);
        }
    };

    println!("This will publish version 0.0.0 of package `{package}` to crates.io.");
    println!("Please make sure that this is what you want.");
    println!("Type \"Yes Daddy 🥺\" (including the emoji) to continue.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() != "Yes Daddy 🥺" {
        eprintln!("Incorrect input, exiting.");
        exit(1);
    }

    let dir = tempdir()?;
    env::set_current_dir(&dir)?;

    fs::write(
        "Cargo.toml",
        format!(
            r#"
[package]
name = "{package}"
description = "{package}"
version = "0.0.0"
license-file = "LICENSE"

[lib]
path = "lib.rs"
"#
        ),
    )?;
    fs::write("lib.rs", "")?;
    fs::write("LICENSE", "")?;

    let cargo = cargo_path();
    let status = Command::new(cargo).arg("publish").status()?;
    if !status.success() {
        eprintln!("`cargo publish` failed");
        exit(1);
    }

    Ok(())
}

fn cargo_path() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}
