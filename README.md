# ASCII Art CLI 🎨

Convert images to beautiful colorized ASCII art directly in your terminal!

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-2021-orange)

## ✨ Features

- **Automatic Terminal Width Detection** - Fits ASCII art perfectly to your terminal
- **Two Rendering Modes**:
  - **Standard Mode**: Brightness-based ASCII conversion for detailed representations
  - **Edge Detection Mode**: Canny edge detection for contour-based artistic output
- **ANSI Colorization**: Full 16-color ANSI support for vibrant, colorful output
- **Multiple Image Formats**: Supports PNG and JPEG images
- **Smart Aspect Ratio Correction**: Accounts for terminal character dimensions (2:1 ratio)
- **User-Friendly CLI**: Simple command-line interface with sensible defaults

## 📸 Examples

### Standard Mode
```bash
cargo run -- examples/black-and-white.jpg --width 80
```
Converts images using brightness-to-ASCII mapping with colors sampled from the original image.

### Edge Detection Mode
```bash
cargo run -- examples/black-and-white.jpg --mode edge --width 60
```
Applies Canny edge detection for a clean, contour-based representation.

## �� Installation

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo package manager

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ascii-art-cli.git
cd ascii-art-cli

# Build the project
cargo build --release

# Run the binary
./target/release/ascii-art-cli examples/black-and-white.jpg
```

## 📖 Usage

```bash
ascii-art-cli [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Path to the input image file (PNG or JPEG)

Options:
      --width <WIDTH>  Override the output width (characters)
      --mode <MODE>    Rendering mode: "standard" or "edge" [default: standard]
  -h, --help           Print help
  -V, --version        Print version
```

### Examples

```bash
# Auto-detect terminal width (recommended)
cargo run -- image.png

# Custom width
cargo run -- image.jpg --width 100

# Edge detection mode
cargo run -- photo.png --mode edge

# Combine options
cargo run -- landscape.jpg --mode edge --width 120
```

## 🛠️ Technical Details

### Architecture
The project is organized into focused modules:
- **`terminal.rs`** - Terminal width detection and utilities
- **`image_loader.rs`** - Image loading and preprocessing
- **`ascii_converter.rs`** - Standard brightness-to-ASCII conversion
- **`edge_detector.rs`** - Canny edge detection-based conversion
- **`renderer.rs`** - ANSI colorization and terminal rendering

### Dependencies
- **`image`** (v0.24) - Image loading and processing
- **`imageproc`** (v0.23) - Canny edge detection algorithm
- **`clap`** (v4) - Command-line argument parsing
- **`terminal-size`** (v0.3) - Cross-platform terminal dimension detection

### Algorithm Highlights
- **Brightness Mapping**: Maps pixel brightness (0-255) to 10-character density scale
- **Edge Detection**: Uses Canny algorithm with thresholds (low=50.0, high=100.0)
- **Color Matching**: Euclidean distance in RGB space to find closest ANSI color
- **Aspect Correction**: 2:1 height adjustment for terminal character dimensions

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test suite
cargo test ascii_conversion
cargo test edge_detection
```

**Test Coverage**: 
- 21 unit tests covering core functionality
- 9 integration tests for CLI behavior
- Manual testing documented for visual verification

## 📋 Project Structure

```
ascii-art-cli/
├── src/
│   ├── main.rs              # Entry point & CLI orchestration
│   ├── terminal.rs          # Terminal utilities
│   ├── image_loader.rs      # Image loading & preprocessing
│   ├── ascii_converter.rs   # Standard brightness conversion
│   ├── edge_detector.rs     # Edge detection mode
│   └── renderer.rs          # ANSI colorization & rendering
├── tests/
│   ├── ascii_conversion.rs  # Integration tests
│   ├── cli_width.rs         # CLI argument tests
│   └── edge_detection.rs    # Edge mode tests
├── examples/
│   └── black-and-white.jpg  # Sample test image
├── Cargo.toml
└── README.md
```

## 🎯 Features & Roadmap

### ✅ Completed (MVP)
- [x] Terminal width auto-detection
- [x] PNG/JPEG image loading
- [x] Brightness-based ASCII conversion
- [x] Canny edge detection mode
- [x] ANSI colorization (16 colors)
- [x] CLI interface with clap
- [x] Comprehensive error handling

### 🔮 Future Enhancements
- [ ] 256-color ANSI support
- [ ] True color (24-bit RGB) mode
- [ ] GIF support with animation
- [ ] Custom character sets
- [ ] Output to file (HTML, ANSI text)
- [ ] Dithering algorithms
- [ ] Interactive mode with live preview
- [ ] Custom edge detection thresholds

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test`)
4. Run clippy (`cargo clippy`)
5. Format code (`cargo fmt`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- **Image Processing**: Built with the excellent `image` and `imageproc` crates
- **CLI Framework**: Powered by `clap` for robust argument parsing
- **Inspiration**: Classic ASCII art converters and terminal-based graphics

Project Link: [https://github.com/yourusername/ascii-art-cli](https://github.com/yourusername/ascii-art-cli)

---

Made with ❤️ using Rust
