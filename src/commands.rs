//! Parse command by [clap]

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Commands {
    /// The file is specified to run
    pub file_path: Option<String>
}