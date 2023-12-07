//! Parse command by [clap]

use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Commands {
    pub file_path: Option<String>
}