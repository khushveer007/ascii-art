use image::{DynamicImage, GenericImageView};
use crate::ascii_converter::AsciiGrid;

// 16 basic ANSI colors (foreground codes)
const ANSI_COLORS: [(u8, u8, u8, &str); 16] = [
    (0, 0, 0, "\x1b[30m"),         // Black
    (128, 0, 0, "\x1b[31m"),       // Red
    (0, 128, 0, "\x1b[32m"),       // Green
    (128, 128, 0, "\x1b[33m"),     // Yellow
    (0, 0, 128, "\x1b[34m"),       // Blue
    (128, 0, 128, "\x1b[35m"),     // Magenta
    (0, 128, 128, "\x1b[36m"),     // Cyan
    (192, 192, 192, "\x1b[37m"),   // White
    (128, 128, 128, "\x1b[90m"),   // Bright Black (Gray)
    (255, 0, 0, "\x1b[91m"),       // Bright Red
    (0, 255, 0, "\x1b[92m"),       // Bright Green
    (255, 255, 0, "\x1b[93m"),     // Bright Yellow
    (0, 0, 255, "\x1b[94m"),       // Bright Blue
    (255, 0, 255, "\x1b[95m"),     // Bright Magenta
    (0, 255, 255, "\x1b[96m"),     // Bright Cyan
    (255, 255, 255, "\x1b[97m"),   // Bright White
];

const RESET: &str = "\x1b[0m";

/// Map RGB values to the closest ANSI color code using Euclidean distance
pub fn rgb_to_ansi(r: u8, g: u8, b: u8) -> String {
    let mut min_distance = f32::MAX;
    let mut closest_code = ANSI_COLORS[0].3;
    
    for &(ar, ag, ab, code) in &ANSI_COLORS {
        let distance = (
            (r as f32 - ar as f32).powi(2) +
            (g as f32 - ag as f32).powi(2) +
            (b as f32 - ab as f32).powi(2)
        ).sqrt();
        
        if distance < min_distance {
            min_distance = distance;
            closest_code = code;
        }
    }
    
    closest_code.to_string()
}

/// Render ASCII grid to terminal with colors from original image
pub fn render_colored(
    grid: &AsciiGrid,
    original: &DynamicImage,
) -> Result<(), String> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            // Sample original pixel color
            let pixel = original.get_pixel(x as u32, y as u32);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            
            // Get ANSI color code
            let color_code = rgb_to_ansi(r, g, b);
            
            // Print colored character
            print!("{}{}", color_code, ch);
        }
        // Reset color at end of line
        println!("{}", RESET);
    }
    
    // Final reset for terminal state safety
    print!("{}", RESET);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_ansi_black() {
        assert_eq!(rgb_to_ansi(0, 0, 0), "\x1b[30m");
    }

    #[test]
    fn test_rgb_to_ansi_white() {
        assert_eq!(rgb_to_ansi(255, 255, 255), "\x1b[97m");
    }

    #[test]
    fn test_rgb_to_ansi_bright_red() {
        assert_eq!(rgb_to_ansi(255, 0, 0), "\x1b[91m");
    }

    #[test]
    fn test_rgb_to_ansi_closest_match() {
        // Test that (250, 250, 250) maps to bright white (closest to 255,255,255)
        assert_eq!(rgb_to_ansi(250, 250, 250), "\x1b[97m");
        
        // Test that (130, 0, 0) maps to red (closest to 128,0,0)
        assert_eq!(rgb_to_ansi(130, 0, 0), "\x1b[31m");
    }
}
