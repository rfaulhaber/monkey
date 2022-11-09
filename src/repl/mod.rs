use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Repl;

impl Repl {
    pub fn new() -> Repl {
        Repl {}
    }

    pub fn run(&self) -> Result<()> {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new()?;
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    let lexer = Lexer::new(&line);
                    lexer.for_each(|tok| println!("token: {:?}", tok));
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break Ok(());
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break Ok(());
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break Ok(());
                }
            }
        }
    }
}
