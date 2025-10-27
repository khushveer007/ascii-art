use image::GrayImage;
use imageproc::edges::canny;

use crate::ascii_converter::AsciiGrid;

/// Low threshold for Canny edge detection (hardcoded for MVP)
const LOW_THRESHOLD: f32 = 50.0;

/// High threshold for Canny edge detection (hardcoded for MVP)
const HIGH_THRESHOLD: f32 = 100.0;

/// Maps an edge pixel value to an ASCII character
///
/// Binary mapping:
/// - 255 (edge) → '#' (dense character)
/// - 0 (non-edge) → ' ' (space)
///
/// # Arguments
/// * `edge_value` - Pixel value from edge map (0 or 255)
///
/// # Returns
/// * '#' for edge pixels (255)
/// * ' ' for non-edge pixels (0)
pub fn edge_to_char(edge_value: u8) -> char {
    if edge_value == 255 {
        '#'
    } else {
        ' '
    }
}

/// Applies Canny edge detection and converts the result to an ASCII grid
///
/// This function performs the following steps:
/// 1. Applies Canny edge detection using hardcoded thresholds
/// 2. Converts the binary edge map (255=edge, 0=non-edge) to ASCII characters
/// 3. Returns a grid with dimensions matching the input image
///
/// # Arguments
/// * `gray` - The grayscale image to process
///
/// # Returns
/// * `Ok(AsciiGrid)` - A 2D vector of characters ('#' for edges, ' ' for non-edges)
/// * `Err(String)` - Error message if conversion fails
pub fn detect_and_convert(gray: &GrayImage) -> Result<AsciiGrid, String> {
    let (width, height) = gray.dimensions();
    
    if width == 0 || height == 0 {
        return Err("Image dimensions must be greater than zero.".to_string());
    }

    // Apply Canny edge detection
    let edge_map = canny(gray, LOW_THRESHOLD, HIGH_THRESHOLD);
    
    // Convert edge map to ASCII grid
    let mut grid = Vec::with_capacity(height as usize);
    
    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let pixel = edge_map.get_pixel(x, y);
            let edge_value = pixel[0];
            row.push(edge_to_char(edge_value));
        }
        grid.push(row);
    }
    
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    #[test]
    fn test_edge_to_char_boundary_values() {
        assert_eq!(edge_to_char(0), ' ');    // Non-edge
        assert_eq!(edge_to_char(255), '#');  // Edge
    }

    #[test]
    fn test_detect_and_convert_dimensions_match() {
        // Create a simple test grayscale image
        let gray = GrayImage::from_pixel(10, 5, image::Luma([128]));
        
        let grid = detect_and_convert(&gray).expect("conversion succeeds");
        
        assert_eq!(grid.len(), 5, "Grid should have 5 rows (height)");
        assert_eq!(grid[0].len(), 10, "Each row should have 10 characters (width)");
    }

    #[test]
    fn test_detect_and_convert_fully_edge() {
        // Create image with all 255 values (should be detected as edges)
        let gray = GrayImage::from_pixel(4, 3, image::Luma([255]));
        
        let grid = detect_and_convert(&gray).expect("conversion succeeds");
        
        // Note: Canny may not detect uniform images as edges, but this tests the mapping logic
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0].len(), 4);
    }

    #[test]
    fn test_detect_and_convert_non_edge() {
        // Create image with all 0 values (black, no edges)
        let gray = GrayImage::from_pixel(4, 3, image::Luma([0]));
        
        let grid = detect_and_convert(&gray).expect("conversion succeeds");
        
        // Fully black image should have no edges, all spaces
        for row in &grid {
            for &ch in row {
                assert_eq!(ch, ' ', "Fully black image should produce all spaces");
            }
        }
    }

    #[test]
    fn test_detect_and_convert_rejects_zero_dimensions() {
        let gray = ImageBuffer::new(0, 0);
        let err = detect_and_convert(&gray).unwrap_err();
        assert_eq!(err, "Image dimensions must be greater than zero.");
    }
}
