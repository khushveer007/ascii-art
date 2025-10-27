use terminal_size::{terminal_size, Height, Width};

/// Indicates how the final output width was decided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthSource {
    User,
    AutoDetected,
    Fallback,
}

/// Result of resolving the output width, including provenance information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthResolution {
    pub width: u32,
    pub source: WidthSource,
}

/// Return the terminal size (width, height) in characters if detection succeeds.
pub fn get_terminal_size() -> Option<(u32, u32)> {
    terminal_size().map(|(Width(w), Height(h))| (u32::from(w), u32::from(h)))
}

pub fn resolve_output_width(user_width: Option<u32>) -> WidthResolution {
    let detected_width = get_terminal_size().map(|(w, _)| w);
    compute_output_width(user_width, detected_width)
}

fn compute_output_width(user_width: Option<u32>, detected_width: Option<u32>) -> WidthResolution {
    match user_width {
        Some(width) => WidthResolution {
            width,
            source: WidthSource::User,
        },
        None => {
            let width_with_margin = detected_width
                .map(apply_margin)
                .map(|width| width.max(40))
                .unwrap_or(80);

            let source = if detected_width.is_some() {
                WidthSource::AutoDetected
            } else {
                WidthSource::Fallback
            };

            WidthResolution {
                width: width_with_margin,
                source,
            }
        }
    }
}

fn apply_margin(width: u32) -> u32 {
    width.saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_width_override() {
        let resolution = compute_output_width(Some(120), Some(100));
        assert_eq!(resolution.width, 120);
        assert_eq!(resolution.source, WidthSource::User);
    }

    #[test]
    fn test_minimum_width_enforced() {
        let resolution = compute_output_width(None, Some(30));
        assert_eq!(resolution.width, 40);
        assert_eq!(resolution.source, WidthSource::AutoDetected);

        let fallback = compute_output_width(None, None);
        assert_eq!(fallback.width, 80);
        assert_eq!(fallback.source, WidthSource::Fallback);
    }
}
