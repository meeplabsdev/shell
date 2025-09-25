// This file is GENERATED (build.rs)

use crate::shell::Shell;
use crate::stringbuffer::StringBuffer;

#[cfg(test)]
pub mod r#false;
pub mod r#noop;
pub mod r#pwd;

#[allow(dead_code)]
pub struct BuiltinResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[allow(dead_code)]
pub fn std_builtin<S: AsRef<str>>(builtin: S, arguments: Vec<String>) -> BuiltinResult {
    let stdin = StringBuffer::new();
    let stdout = StringBuffer::new();
    let stderr = StringBuffer::new();

    let mut shell = Shell::new(stdin, stdout.clone(), stderr.clone());
    let command = shell.builtin(&builtin.as_ref().to_string()).unwrap();
    let exit_code = command(&mut shell, arguments);

    return BuiltinResult {
        exit_code,
        stdout: stdout.into(),
        stderr: stderr.into(),
    };
}
