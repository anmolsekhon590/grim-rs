use clap::{Parser, ValueEnum};

/// Grim re-write in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Scale factor (e.g., 1.5 for 150% scaling)
    #[arg(short = 's', long = "scale", default_value = "1.0")]
    pub scale: f64,

    /// Capture geometry (region), reads from stdin if '-g -' is passed
    #[arg(short = 'g', long = "geometry")]
    pub geometry: Option<String>,

    /// Output file type (png, ppm, jpeg)
    #[arg(short = 't', long = "type", default_value = "png")]
    pub filetype: FileType,

    /// JPEG filetype quality
    #[arg(short = 'q', long = "quality", default_value = "80")]
    pub quality: u8,

    /// PNG compression level
    #[arg(short = 'l', long = "level", default_value = "6")]
    pub level: u8,

    /// Output name to Capture
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum FileType {
    Png,
    Ppm,
    Jpeg,
}
