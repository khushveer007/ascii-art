use std::fmt;
use std::io;

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, GrayImage, ImageError};

/// Bundle of image data prepared for downstream conversion/rendering stages.
#[derive(Debug)]
pub struct ProcessedImage {
    pub gray: GrayImage,
    pub original: DynamicImage,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ImageLoaderError {
    FileNotFound(String),
    UnsupportedFormat(String),
    InvalidDimensions(String),
    DecodeFailed(String),
    IoError(String),
}

impl fmt::Display for ImageLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageLoaderError::FileNotFound(path) => {
                write!(f, "Could not find image file \"{}\".", path)
            }
            ImageLoaderError::UnsupportedFormat(path) => {
                write!(f, "Unsupported image format for file \"{}\".", path)
            }
            ImageLoaderError::InvalidDimensions(message) => write!(f, "{message}"),
            ImageLoaderError::DecodeFailed(message) => write!(f, "{message}"),
            ImageLoaderError::IoError(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for ImageLoaderError {}

pub fn load_image(path: &str) -> Result<DynamicImage, ImageLoaderError> {
    image::open(path).map_err(|err| map_image_error(err, path))
}

pub fn preprocess_image(
    img: DynamicImage,
    target_width: u32,
) -> Result<ProcessedImage, ImageLoaderError> {
    if target_width == 0 {
        return Err(ImageLoaderError::InvalidDimensions(
            "Target width must be greater than zero.".to_string(),
        ));
    }

    let (original_width, original_height) = img.dimensions();
    if original_width == 0 || original_height == 0 {
        return Err(ImageLoaderError::InvalidDimensions(
            "Input image has invalid dimensions.".to_string(),
        ));
    }

    let corrected_width = target_width;
    let aspect_ratio = original_height as f32 / original_width as f32;
    let target_height = ((aspect_ratio * corrected_width as f32) / 2.0)
        .round()
        .max(1.0) as u32;

    let resized = img.resize_exact(corrected_width, target_height, FilterType::Lanczos3);
    let gray = image::imageops::grayscale(&resized);

    Ok(ProcessedImage {
        original: resized,
        gray,
    })
}

fn map_image_error(error: ImageError, path: &str) -> ImageLoaderError {
    match error {
        ImageError::IoError(io_err) => map_io_error(io_err, path),
        ImageError::Unsupported(_) => ImageLoaderError::UnsupportedFormat(path.to_string()),
        ImageError::Decoding(err) => {
            ImageLoaderError::DecodeFailed(format!("Failed to decode image \"{path}\": {err}"))
        }
        ImageError::Limits(err) => {
            ImageLoaderError::DecodeFailed(format!("Image limits exceeded for \"{path}\": {err}"))
        }
        other => {
            ImageLoaderError::DecodeFailed(format!("Failed to load image \"{path}\": {other}"))
        }
    }
}

fn map_io_error(error: io::Error, path: &str) -> ImageLoaderError {
    match error.kind() {
        io::ErrorKind::NotFound => ImageLoaderError::FileNotFound(path.to_string()),
        _ => ImageLoaderError::IoError(format!("I/O error while accessing \"{path}\": {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    #[test]
    fn load_image_missing_file() {
        let err = load_image("tests/data/does_not_exist.png").unwrap_err();
        assert_eq!(
            err,
            ImageLoaderError::FileNotFound("tests/data/does_not_exist.png".to_string())
        );
        assert_eq!(
            err.to_string(),
            "Could not find image file \"tests/data/does_not_exist.png\"."
        );
    }

    #[test]
    fn preprocess_image_respects_aspect_ratio_and_width() {
        let image =
            DynamicImage::ImageRgba8(ImageBuffer::from_pixel(4, 4, Rgba([200, 100, 50, 255])));

        let processed = preprocess_image(image, 80).expect("preprocess succeeds");
        assert_eq!(processed.original.dimensions(), (80, 40));
        assert_eq!(processed.gray.dimensions(), (80, 40));
    }

    #[test]
    fn preprocess_image_rejects_zero_width() {
        let image = DynamicImage::ImageRgba8(ImageBuffer::from_pixel(4, 4, Rgba([0, 0, 0, 255])));
        let err = preprocess_image(image, 0).unwrap_err();
        assert_eq!(
            err,
            ImageLoaderError::InvalidDimensions("Target width must be greater than zero.".into())
        );
    }
}
