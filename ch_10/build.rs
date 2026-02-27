use std::{borrow::Cow, process::Command};

/// Generate the `cargo:` key output
pub fn generate_cargo_keys() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    let commit = match output {
        Ok(o) if o.status.success() => {
            let sha = String::from_utf8_lossy(&o.stdout).trim().to_owned();
            Cow::from(sha)
        }
        Ok(o) => {
            println!(
                "cargo:warning=Git command failed with status: {}",
                o.status
            );
            Cow::from("unknown")
        }
        Err(err) => {
            println!(
                "cargo:warning=Failed to execute git command: {}",
                err
            );
            Cow::from("unknown")
        }
    };

    println!(
        "cargo:rustc-env=RUST_WEB_DEV_VERSION={}",
        get_version(&commit)
    )
}

fn get_platform() -> String {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let env_dash = if env.is_empty() { "" } else { "-" };

    format!("{}-{}{}{}", arch, os, env_dash, env)
}

fn get_version(impl_commit: &str) -> String {
    let commit_dash = if impl_commit.is_empty() { "" } else { "-" };

    format!(
        "{}{}{}-{}",
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
        commit_dash,
        impl_commit,
        get_platform(),
    )
}

fn main() {
    generate_cargo_keys();
}
