use clap::Parser;
use monkey::{
    cli::{Cli, Command},
    repl::Repl,
};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Repl) => {
            let repl = Repl::new();
            let _ = repl.run();
        }
        _ => println!("invalid command!"),
    }
}
