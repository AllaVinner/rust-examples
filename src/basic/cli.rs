use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum BasicCommands {
    /// Say hi to someone
    Hello {
        /// How do you want to say hi to?
        #[arg(short, long)]
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
