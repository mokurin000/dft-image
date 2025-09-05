use std::path::PathBuf;

use palc::Parser;

#[derive(Clone, Debug, Parser)]
pub struct CliArgs {
    #[arg(long, short = 'F', default_value_t = 15.0)]
    pub factor: f64,
    #[arg(long)]
    pub skip_shift: bool,
    /// Input image path, file extension does not matter
    pub input_path: PathBuf,
    /// Output image path, must with a file extension
    pub output_path: PathBuf,
}
