use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_edge_mode_produces_output() {
    // Test that edge mode runs successfully with colored output
    Command::cargo_bin("ascii-art-cli")
        .unwrap()
        .arg("examples/test_image_1.png")
        .arg("--mode")
        .arg("edge")
        .arg("--width")
        .arg("40")
        .assert()
        .success()
        .stdout(predicate::str::contains("\x1b[")); // Check for ANSI escape codes
}

#[test]
fn test_edge_mode_invalid_mode() {
    // Test that invalid mode produces error
    Command::cargo_bin("ascii-art-cli")
        .unwrap()
        .arg("examples/test_image_1.png")
        .arg("--mode")
        .arg("invalid")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unknown mode 'invalid'"));
}

#[test]
fn test_standard_mode_still_works() {
    // Ensure standard mode continues to work
    Command::cargo_bin("ascii-art-cli")
        .unwrap()
        .arg("examples/test_image_1.png")
        .arg("--mode")
        .arg("standard")
        .arg("--width")
        .arg("40")
        .assert()
        .success();
}

#[test]
fn test_default_mode_is_standard() {
    // Test that omitting --mode defaults to standard
    Command::cargo_bin("ascii-art-cli")
        .unwrap()
        .arg("examples/test_image_1.png")
        .arg("--width")
        .arg("40")
        .assert()
        .success();
}
