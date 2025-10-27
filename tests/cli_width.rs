use assert_cmd::Command;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgba};
use predicates::str::contains;
use tempfile::NamedTempFile;

#[test]
fn help_lists_width_flag() {
    Command::cargo_bin("ascii-art-cli")
        .expect("binary exists")
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("--width"));
}

#[test]
fn run_with_width_override_succeeds() {
    let image_file = create_sample_image();

    Command::cargo_bin("ascii-art-cli")
        .expect("binary exists")
        .arg(image_file.path())
        .arg("--width")
        .arg("80")
        .assert()
        .success()
        .stdout(contains("\x1b[")); // Check for ANSI escape codes
}

#[test]
fn missing_image_reports_user_friendly_error() {
    Command::cargo_bin("ascii-art-cli")
        .expect("binary exists")
        .arg("tests/data/does-not-exist.png")
        .assert()
        .failure()
        .stderr(contains(
            "Could not find image file \"tests/data/does-not-exist.png\".",
        ));
}

fn create_sample_image() -> NamedTempFile {
    let mut file = NamedTempFile::with_suffix(".png").expect("create temp image file");
    let image = ImageBuffer::from_fn(4, 4, |x, y| {
        let r = (x * 40) as u8;
        let g = (y * 60) as u8;
        let b = 150u8;
        Rgba([r, g, b, 255])
    });

    DynamicImage::ImageRgba8(image)
        .write_to(&mut file, ImageOutputFormat::Png)
        .expect("write png");

    file
}
