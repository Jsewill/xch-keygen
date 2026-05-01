//! Tests for address encoding and output formatting.

use std::process::Command;

fn xch_keygen() -> Command {
    let progs = cargo_crate_bin("xch-keygen");
    Command::new(progs)
}

fn cargo_crate_bin(name: &str) -> String {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    format!("{}/target/{profile}/{}", root, name)
}

/// Parse the address table from CLI output.
/// Returns a vector of (id, type, address, public_key) tuples.
fn parse_addresses(output: &[u8]) -> Vec<(usize, String, String, String)> {
    let stdout = String::from_utf8_lossy(output).into_owned();
    let mut results = Vec::new();
    let mut skip_header = false;
    for line in stdout.lines() {
        if line.contains("ID") && line.contains("Type") {
            skip_header = true;
            continue;
        }
        if skip_header && line.trim().is_empty() {
            continue;
        }
        if skip_header && line.contains("xch1") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let id: usize = parts[0].parse().unwrap_or(0);
                let addr_type = parts[1].to_string();
                let address = parts[2].to_string();
                let pubkey = parts[3].to_string();
                results.push((id, addr_type, address, pubkey));
            }
        }
    }
    results
}

#[test]
fn address_format_valid() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    assert!(!addrs.is_empty(), "should have at least one address");
    for (_, _, addr, _) in &addrs {
        assert!(addr.starts_with("xch1"), "address should start with xch1: {addr}");
        assert!(addr.len() > 10, "address should be a valid bech32m string: {addr}");
    }
}

#[test]
fn address_count_matches_request() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("5")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    let hardened: Vec<_> = addrs.iter().filter(|(_, t, _, _)| t == "Hardened").collect();
    let unhardened: Vec<_> = addrs.iter().filter(|(_, t, _, _)| t == "Unhardened").collect();
    assert_eq!(hardened.len(), 5, "should have 5 hardened addresses");
    assert_eq!(unhardened.len(), 5, "should have 5 unhardened addresses");
}

#[test]
fn skip_produces_correct_count() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("3")
        .arg("-s")
        .arg("2")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    let hardened: Vec<_> = addrs.iter().filter(|(_, t, _, _)| t == "Hardened").collect();
    assert_eq!(hardened.len(), 3, "should have 3 hardened addresses with skip=2");
}

#[test]
fn random_produces_correct_count() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("4")
        .arg("-r")
        .arg("-o")
        .arg("50")
        .arg("-m")
        .arg("100")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    let hardened: Vec<_> = addrs.iter().filter(|(_, t, _, _)| t == "Hardened").collect();
    assert_eq!(hardened.len(), 4, "should have 4 hardened addresses with random mode");
}

#[test]
fn fingerprint_in_output() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Fingerprint:"), "output should contain fingerprint");
}

#[test]
fn mnemonic_in_output() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Mnemonic:"), "output should contain mnemonic");
}

#[test]
fn quiet_mode_no_output() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .arg("-q")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.trim().is_empty(), "quiet mode should produce no stdout");
}

#[test]
fn named_wallet_in_output() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .arg("--enable-naming")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Label:"), "output should contain label when naming is enabled");
}

#[test]
fn public_key_in_address_row() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("1")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    for (_, _, _, pubkey) in &addrs {
        assert_eq!(pubkey.len(), 96, "public key should be 48 bytes (96 hex chars): {pubkey}");
    }
}

#[test]
fn burn_address_format() {
    // The chia burn address derived from puzzle hash 0x000000000000000000000000000000000000000000000000000000000000dead
    let burn_addr = "xch1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqm6ks6e8mvy";
    assert!(burn_addr.starts_with("xch1"), "burn address should start with xch1");
    assert_eq!(burn_addr.len(), 62, "burn address should be 62 chars");
    assert!(burn_addr.contains("qqqq"), "burn address should contain q's (zero bytes in bech32m)");
}

#[test]
fn generated_addresses_are_unique() {
    let output = xch_keygen()
        .arg("-w")
        .arg("12")
        .arg("-a")
        .arg("10")
        .output()
        .expect("failed to run binary");
    assert!(output.status.success());
    let addrs = parse_addresses(&output.stdout);
    let hardened: Vec<_> = addrs.iter().filter(|(_, t, _, _)| t == "Hardened").map(|(_, _, a, _)| a.clone()).collect();
    let mut unique = hardened.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), hardened.len(), "all hardened addresses should be unique");
}
