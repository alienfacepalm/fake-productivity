use font_kit::source::SystemSource;
use font_kit::handle::Handle;
use std::path::Path;

pub fn load_custom_font() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the font file
    let font_path = Path::new("assets/fonts/DroidSansMonoDottedForPowerline.ttf");

    // Load the font
    let font_handle = SystemSource::new().select_by_postscript_name("DroidSansMonoDottedForPowerline")
        .or_else(|_| Ok::<Handle, Box<dyn std::error::Error>>(Handle::from_path(font_path.to_path_buf(), 0)))?;

    // Use the font (example: print its PostScript name)
    if let Handle::Path { path, .. } = font_handle {
        println!("Loaded font from path: {:?}", path);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_custom_font() {
        let result = load_custom_font();
        // It might fail in CI/headless if font file missing, but we want to know if it compiles and runs.
        // The original test asserted is_ok(), so we keep that.
        // Note: we need to make sure the relative path "assets/fonts/..." resolves during test execution.
        // Cargo runs tests with CWD as the package root, so it should be fine.
        assert!(result.is_ok(), "Failed to load custom font: {:?}", result.err());
    }
}
