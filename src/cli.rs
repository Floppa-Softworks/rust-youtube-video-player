use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    Play {
        url: Option<String>
    },
}
