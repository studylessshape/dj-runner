//! Parse command by [clap]

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub struct Commands {
    /// The file is specified to run
    #[arg(short, long)]
    pub file_path: Option<String>,
    /// Enable cut input when press Enter
    #[arg(short, long, default_value_t = false)]
    pub cut_input: bool
}
