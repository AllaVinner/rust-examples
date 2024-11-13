use crate::basic;
use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
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
