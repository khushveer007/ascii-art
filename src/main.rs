use clap::Parser;

mod ascii_converter;
mod edge_detector;
mod image_loader;
mod renderer;
mod terminal;

use crate::ascii_converter::{convert_to_ascii, AsciiGrid};
use crate::image_loader::{load_image, preprocess_image, ProcessedImage};
use terminal::WidthSource;

#[derive(Debug, Parser)]
#[command(
    name = "ascii-art-cli",
    version,
    author,
    about = "Convert images to colorized ASCII art in the terminal",
    long_about = None
)]
struct Cli {
    /// Path to the input image file (PNG or JPEG)
    #[arg(value_name = "IMAGE")]
    image_path: String,

    /// Override the output width (characters)
    #[arg(long)]
    width: Option<u32>,

    /// Rendering mode: "standard" or "edge"
    #[arg(long, default_value = "standard")]
    mode: String,
}

fn main() {
    let cli = Cli::parse();
    let width_resolution = terminal::resolve_output_width(cli.width);

    emit_width_messages(width_resolution.source, width_resolution.width);

    match run_pipeline(&cli, width_resolution.width) {
        Ok((processed, ascii_grid)) => {
            // Render colored ASCII art to terminal
            if let Err(e) = renderer::render_colored(&ascii_grid, &processed.original) {
                eprintln!("Rendering error: {}", e);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

fn emit_width_messages(source: WidthSource, width: u32) {
    match source {
        WidthSource::User => { /* User override already explicit. */ }
        WidthSource::AutoDetected => {
            println!("Using auto-detected width: {width} characters");
        }
        WidthSource::Fallback => {
            eprintln!("Warning: Unable to detect terminal size; defaulting to {width} characters.");
            println!("Using fallback width: {width} characters");
        }
    }
}

fn run_pipeline(cli: &Cli, width: u32) -> Result<(ProcessedImage, AsciiGrid), String> {
    let image = load_image(&cli.image_path).map_err(|e| e.to_string())?;
    let processed = preprocess_image(image, width).map_err(|e| e.to_string())?;
    
    // Select conversion mode based on CLI argument
    let ascii_grid = match cli.mode.as_str() {
        "edge" => edge_detector::detect_and_convert(&processed.gray)
            .map_err(|e| format!("Edge detection failed: {}", e))?,
        "standard" => convert_to_ascii(&processed.gray)?,
        unknown => return Err(format!("Unknown mode '{}'. Use 'standard' or 'edge'.", unknown)),
    };
    
    Ok((processed, ascii_grid))
}
