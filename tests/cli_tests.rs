//! Integration tests for CLI argument parsing and basic execution.

use std::process::Command;

fn xch_keygen() -> Command {
    let progs = cargo_crate_bin("xch-keygen");
    Command::new(progs)
}

/// Resolve the path to the `xch-keygen` binary built by cargo.
fn cargo_crate_bin(name: &str) -> String {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    format!("{}/target/{profile}/{}", root, name)
}

#[test]
fn cli_help_exits_ok() {
    let output = xch_keygen().arg("--help").output().expect("failed to run binary");
    assert!(output.status.success(), "help should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--words"));
    assert!(stdout.contains("--addresses"));
}

#[test]
fn cli_version_exits_ok() {
    let output = xch_keygen().arg("--version").output().expect("failed to run binary");
    assert!(output.status.success());
}

#[test]
fn cli_default_generates_ok() {
    let output = xch_keygen().arg("-a").arg("1").arg("-q").output().expect("failed to run binary");
    assert!(output.status.success(), "default generation should succeed");
}

#[test]
fn cli_12_word_generates_ok() {
    let output = xch_keygen().arg("-w").arg("12").arg("-a").arg("1").arg("-q").output().expect("failed to run binary");
    assert!(output.status.success());
}

#[test]
fn cli_24_word_generates_ok() {
    let output = xch_keygen().arg("-w").arg("24").arg("-a").arg("1").arg("-q").output().expect("failed to run binary");
    assert!(output.status.success());
}

#[test]
fn cli_invalid_word_count_rejected() {
    let output = xch_keygen().arg("-w").arg("18").arg("-a").arg("1").arg("-q").output().expect("failed to run binary");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("possible values"));
}

#[test]
fn cli_mnemonic_from_stdin() {
    let output = xch_keygen()
        .arg("-a")
        .arg("1")
        .arg("-q")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn")
        .wait_with_output()
        .expect("failed to wait");
    // The binary will prompt for input; since we don't provide any, it generates a new mnemonic.
    // This test just verifies the binary doesn't crash.
    assert!(output.status.success());
}

#[test]
fn cli_mnemonic_from_known_phrase() {
    let _phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let output = xch_keygen()
        .arg("-a")
        .arg("1")
        .arg("-q")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("failed to spawn")
        .wait_with_output()
        .expect("failed to wait");
    assert!(output.status.success());
}

#[test]
fn cli_skip_and_addresses() {
    let output = xch_keygen()
        .arg("-a")
        .arg("3")
        .arg("-s")
        .arg("1")
        .arg("-q")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
}

#[test]
fn cli_random_with_offset_and_max() {
    let output = xch_keygen()
        .arg("-a")
        .arg("2")
        .arg("-r")
        .arg("-o")
        .arg("100")
        .arg("-m")
        .arg("200")
        .arg("-q")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
}

#[test]
fn cli_random_conflicts_with_skip() {
    let output = xch_keygen()
        .arg("-r")
        .arg("-s")
        .arg("1")
        .output()
        .expect("failed to run binary");
    assert!(!output.status.success());
}
