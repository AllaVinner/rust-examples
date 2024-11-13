use crate::basic;
use clap::{command, Parser, Subcommand};

/// CLI With Some Rust Examples
#[derive(Parser, Debug)]
#[command(version)]
pub struct CLI {
    #[command(subcommand)]
    command: Commands,
}

impl CLI {
    pub fn run(self) {
        self.command.run();
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Some Basic Examples
    Basic {
        #[command(subcommand)]
        command: basic::cli::BasicCommands,
    },
}

impl Commands {
    fn run(self) {
        match self {
            Self::Basic { command } => command.run(),
        }
    }
}
