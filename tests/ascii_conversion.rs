use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn ascii_conversion_produces_colored_output() {
    // Test that colored ASCII art is rendered with ANSI codes
    Command::cargo_bin("ascii-art-cli")
        .expect("binary exists")
        .arg("examples/test_image_1.png")
        .arg("--width")
        .arg("40")
        .assert()
        .success()
        .stdout(contains("\x1b["))  // Contains ANSI escape codes
        .stdout(contains("\x1b[0m"));  // Contains ANSI reset code
}

#[test]
fn ascii_conversion_handles_different_widths() {
    // Test that different widths work with colored output
    Command::cargo_bin("ascii-art-cli")
        .expect("binary exists")
        .arg("examples/test_image_1.png")
        .arg("--width")
        .arg("80")
        .assert()
        .success()
        .stdout(contains("\x1b["))  // Contains ANSI escape codes
        .stdout(contains("\x1b[0m"));  // Contains ANSI reset code
}
