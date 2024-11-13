use clap::{command, Subcommand};

#[derive(Subcommand, Debug)]
pub enum BasicCommands {
    Hello {
        #[arg(long)]
        name: String,
    },
}

impl BasicCommands {
    pub fn run(self) {
        match self {
            Self::Hello { name } => say_hi(&name),
        }
    }
}

fn say_hi(name: &str) {
    println!("Hello {}", name);
}
