use crate::shell::Shell;

pub fn function(shell: &mut Shell, arguments: Vec<String>) -> i32 {
    let command = arguments.join(" ");

    return shell.exec(command);
}
