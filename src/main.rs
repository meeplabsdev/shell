mod builtin;
mod environment;
mod error;
mod ioface;
mod shell;
mod stringbuffer;
mod tests;

use crate::shell::Shell;
use error::Error;
use std::{io, thread};

#[cfg(not(windows))]
use signal_hook::{
    consts::{SIGINT, SIGQUIT, SIGTSTP},
    iterator::Signals,
};

fn main() -> Result<(), Error> {
    let mut shell = Shell::new(io::stdin(), io::stdout(), io::stderr());

    #[cfg(not(windows))]
    {
        let mut signals = Signals::new([SIGINT, SIGQUIT, SIGTSTP]).unwrap();

        thread::spawn(move || {
            let mut shell = Shell::new(io::stdin(), io::stdout(), io::stderr());
            for sig in signals.forever() {
                let common_name = match sig {
                    SIGINT => "CANCEL",
                    SIGQUIT => "DISCONNECT",
                    SIGTSTP => "FORCE STOP",
                    _ => "unknown",
                };

                shell.writeln(format!("signal {}", common_name)).ok();
            }
        });
    }

    let mut exit_code = 0;
    loop {
        let content = shell.prompt(exit_code)?;
        exit_code = shell.exec(content);

        if exit_code == -255 {
            break;
        }
    }

    return Ok(());
}
