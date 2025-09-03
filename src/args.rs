use std::path::PathBuf;

use palc::Parser;

#[derive(Clone, Debug, Parser)]
pub struct CliArgs {
    /// Input image path, file extension does not matter
    pub input_path: PathBuf,
    /// Output image path, must with a file extension
    pub output_path: PathBuf,
}
