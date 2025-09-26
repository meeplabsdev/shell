mod builtin;
mod error;
mod ioface;
mod shell;
mod stringbuffer;
mod tests;

use crate::shell::Shell;
use error::Error;
use signal_hook::{
    consts::{SIGINT, SIGQUIT, SIGTSTP},
    iterator::Signals,
};
use std::{
    io::{self, Read},
    process::{Command, Stdio},
    thread,
};

#[tokio::main()]
async fn main() -> Result<(), Error> {
    let mut signals = Signals::new([SIGINT, SIGQUIT, SIGTSTP]).unwrap();
    let mut shell = Shell::new(io::stdin(), io::stdout(), io::stderr());

    thread::spawn(move || {
        let mut shell = Shell::new(io::stdin(), io::stdout(), io::stderr());
        for sig in signals.forever() {
            let common_name = match sig {
                SIGINT => "CANCEL",
                SIGQUIT => "DISCONNECT",
                SIGTSTP => "FORCE STOP",
                _ => "unknown",
            };

            let _ = shell.writeln(format!("Received signal {}", common_name));
        }
    });

    let mut exit_code = 0;
    for _ in 0..5 {
        let content = shell.prompt(exit_code)?;
        let parts = parse(content);
        if parts.is_err() {
            exit_code = 0;
            continue;
        }

        let (operand, arguments) = parts?;
        let result = act(operand, arguments, &mut shell);

        if result.is_err() {
            shell.writeln(format!("{}", result.unwrap_err()))?;
            continue;
        }

        exit_code = result?;
    }

    return Ok(());
}

fn parse<S: AsRef<str>>(input: S) -> Result<(String, Vec<String>), Error> {
    let mut parts: Vec<String> = input
        .as_ref()
        .split(' ')
        .filter(|p| !p.eq(&""))
        .map(|p| p.to_string())
        .collect();

    if parts.len() < 1 {
        return Err("".into());
    } else if parts.len() == 1 {
        return Ok((parts.remove(0), Vec::new()));
    }

    return Ok((parts.remove(0), parts));
}

fn act<S: AsRef<str>>(operand: S, arguments: Vec<S>, shell: &mut Shell) -> Result<i32, Error> {
    let operand = operand.as_ref().to_string();
    let arguments = arguments.iter().map(|a| a.as_ref().to_string());

    if operand.starts_with('!') {
        let operand = operand[1..].to_string();
        let mut child = Command::new(operand)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // let stdin = child.stdin.take().expect("failed to get stdin");
        let mut stdout = child.stdout.take().expect("failed to get stdout");
        let mut stderr = child.stderr.take().expect("failed to get stderr");

        let mut buffer = [0u8; 4096];
        loop {
            let n = stdout.read(&mut buffer)?;
            if n != 0 {
                shell.write_buf(&buffer[..n])?;
            }

            let m = stderr.read(&mut buffer)?;
            if m != 0 {
                shell.err_buf(&buffer[..m])?;
            }

            if n == 0 && m == 0 {
                break;
            }
        }

        return Ok(child.wait()?.code().unwrap_or(255));
    }

    if let Some(command) = shell.builtin(&operand) {
        return Ok(command(shell, arguments.collect()));
    }

    shell.errln(format!("\"{}\" not found", operand))?;
    return Ok(255);
}
