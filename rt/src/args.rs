use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use once_cell::sync::Lazy;

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);

#[derive(Parser)]
#[command(name = "Jank Tracer")]
pub struct Args {
    /// What scene file to load in. Relative to working directory.
    #[arg(default_value_os_t = PathBuf::from("scene.ron"))]
    pub scene: PathBuf,

    /// What format to output the renders in.
    #[arg(short, default_value_t = OutputFormat::Png)]
    pub output_format: OutputFormat,

    /// When this flag is set, a render preview window will not be opened
    #[arg(long, default_value_t = false)]
    pub no_preview: bool,

    /// Apply opened scene's post process pipeline to an image. This disables normal rendering.
    /// Output format is defined by -o
    #[arg(long, value_name = "IMAGE")]
    pub post_process: Option<PathBuf>,
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    Png,
    Ppm,
    Exr,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OutputFormat::Png => "png",
            OutputFormat::Ppm => "ppm",
            OutputFormat::Exr => "exr",
        })
    }
}