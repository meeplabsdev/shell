// This file is GENERATED (build.rs)
use crate::shell::Shell;
use std::io;

#[cfg(test)]
pub mod r#false;

#[allow(dead_code)]
pub fn std_shell() -> Shell {
    return Shell::new(io::stdin(), io::stdout(), io::stderr());
}

#[allow(dead_code)]
pub fn std_builtin<S: AsRef<str>>(builtin: S, arguments: Vec<String>) -> i32 {
    let mut shell = std_shell();
    let command = shell.builtin(&builtin.as_ref().to_string()).unwrap();
    return command(&mut shell, arguments);
}
