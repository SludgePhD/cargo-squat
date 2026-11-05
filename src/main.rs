use std::{
    env,
    error::Error,
    ffi::OsString,
    fs, io,
    process::{self, exit, Command},
};

use tempfile::tempdir;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();
    let package = match &*args {
        [package] | ["cargo-squat", package] => package,
        _ => {
            eprintln!("usage: cargo squat <package-name>");
            process::exit(1);
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
