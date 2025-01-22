use clap::{command, Parser};
use text::*;

pub mod text;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Parser, Debug)]
pub enum Subcommands {
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubcommands),
}
