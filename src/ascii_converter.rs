use image::GrayImage;

/// Type alias for a 2D grid of ASCII characters
pub type AsciiGrid = Vec<Vec<char>>;

/// Character set ordered by visual density from dark (space) to light (@)
const CHARSET: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

/// Maps a brightness value (0-255) to an ASCII character
///
/// The mapping distributes the full brightness range evenly across the 10-character set.
/// Brightness 0 maps to ' ' (darkest), 255 maps to '@' (lightest).
pub fn brightness_to_char(brightness: u8) -> char {
    let index = ((brightness as f32 / 255.0) * 9.0).round() as usize;
    CHARSET[index.min(9)]
}

/// Converts a grayscale image to an ASCII character grid
///
/// Each pixel's brightness is mapped to a character, producing a grid
/// with dimensions matching the input image.
///
/// # Arguments
/// * `gray` - The grayscale image to convert
///
/// # Returns
/// * `Ok(AsciiGrid)` - A 2D vector of characters matching image dimensions
/// * `Err(String)` - Error message if conversion fails
pub fn convert_to_ascii(gray: &GrayImage) -> Result<AsciiGrid, String> {
    let (width, height) = gray.dimensions();
    
    if width == 0 || height == 0 {
        return Err("Image dimensions must be greater than zero.".to_string());
    }

    let mut grid = Vec::with_capacity(height as usize);
    
    for y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let pixel = gray.get_pixel(x, y);
            let brightness = pixel[0];
            row.push(brightness_to_char(brightness));
        }
        grid.push(row);
    }
    
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, ImageBuffer};

    #[test]
    fn brightness_to_char_boundary_values() {
        assert_eq!(brightness_to_char(0), ' ');
        assert_eq!(brightness_to_char(127), '=');
        assert_eq!(brightness_to_char(255), '@');
    }

    #[test]
    fn brightness_to_char_intermediate_values() {
        assert_eq!(brightness_to_char(25), '.');
        assert_eq!(brightness_to_char(50), ':');
        assert_eq!(brightness_to_char(75), '-');
        assert_eq!(brightness_to_char(100), '=');
        assert_eq!(brightness_to_char(130), '+');
        assert_eq!(brightness_to_char(180), '*');
        assert_eq!(brightness_to_char(230), '%');
    }

    #[test]
    fn convert_to_ascii_dimensions_match() {
        let gray = GrayImage::from_pixel(10, 5, image::Luma([128]));
        let grid = convert_to_ascii(&gray).expect("conversion succeeds");
        
        assert_eq!(grid.len(), 5, "Grid should have 5 rows");
        assert_eq!(grid[0].len(), 10, "Each row should have 10 characters");
    }

    #[test]
    fn convert_to_ascii_fully_black_image() {
        let gray = GrayImage::from_pixel(4, 3, image::Luma([0]));
        let grid = convert_to_ascii(&gray).expect("conversion succeeds");
        
        for row in &grid {
            for &ch in row {
                assert_eq!(ch, ' ', "Fully black image should produce all spaces");
            }
        }
    }

    #[test]
    fn convert_to_ascii_fully_white_image() {
        let gray = GrayImage::from_pixel(4, 3, image::Luma([255]));
        let grid = convert_to_ascii(&gray).expect("conversion succeeds");
        
        for row in &grid {
            for &ch in row {
                assert_eq!(ch, '@', "Fully white image should produce all '@' characters");
            }
        }
    }

    #[test]
    fn convert_to_ascii_rejects_zero_dimensions() {
        let gray = ImageBuffer::new(0, 0);
        let err = convert_to_ascii(&gray).unwrap_err();
        assert_eq!(err, "Image dimensions must be greater than zero.");
    }

    #[test]
    fn convert_to_ascii_gradient() {
        // Create a simple gradient: black (0), mid-gray (127), white (255)
        let mut gray = GrayImage::new(3, 1);
        gray.put_pixel(0, 0, image::Luma([0]));
        gray.put_pixel(1, 0, image::Luma([127]));
        gray.put_pixel(2, 0, image::Luma([255]));
        
        let grid = convert_to_ascii(&gray).expect("conversion succeeds");
        
        assert_eq!(grid.len(), 1);
        assert_eq!(grid[0].len(), 3);
        assert_eq!(grid[0][0], ' ');
        assert_eq!(grid[0][1], '=');
        assert_eq!(grid[0][2], '@');
    }
}
