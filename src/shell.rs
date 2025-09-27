use crate::{
    builtin::{self, Sig},
    error::Error,
    ioface::IoFace,
};
use chrono::Local;
use colored::Colorize;
use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

static ALIASES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    ":" => "noop",
    "." => "source",
};

#[derive(Clone)]
pub struct Shell {
    io: Arc<Mutex<IoFace>>,
    builtins: HashMap<String, Sig>,
}

impl Shell {
    pub fn new<I: Read + 'static, O: Write + 'static, E: Write + 'static>(
        stdin: I,
        stdout: O,
        stderr: E,
    ) -> Self {
        Self {
            io: Arc::new(Mutex::new(IoFace::new(stdin, stdout, stderr))),
            builtins: builtin::builtins(),
        }
    }

    pub fn writeln<T: AsRef<str>>(&mut self, content: T) -> Result<(), Error> {
        let content = content.as_ref().to_string();

        return self.io.lock()?.writeln(content);
    }

    pub fn errln<T: AsRef<str>>(&mut self, content: T) -> Result<(), Error> {
        let content = content.as_ref().to_string();

        return self.io.lock()?.errln(content);
    }

    pub fn write_buf(&mut self, buffer: &[u8]) -> Result<(), Error> {
        return self.io.lock()?.write_buf(buffer);
    }

    pub fn err_buf(&mut self, buffer: &[u8]) -> Result<(), Error> {
        return self.io.lock()?.err_buf(buffer);
    }

    pub fn prompt_sigil(&mut self, sigil: String) -> Result<String, Error> {
        self.io.lock()?.write(sigil)?;

        return Ok(self.io.lock()?.read()?);
    }

    pub fn prompt(&mut self, exit_code: i32) -> Result<String, Error> {
        let exit_code = if exit_code == 0 {
            format!("{:0>3}", exit_code).dimmed()
        } else {
            format!("{:0>3}", exit_code.abs()).black().on_white()
        };

        let local = Local::now().format("%H:%M:%S").to_string().dimmed();
        let dir = env::current_dir().unwrap();
        let home = env::home_dir();

        let prompt;
        if let Some(home) = home
            && dir.starts_with(&home)
        {
            let dir = pathdiff::diff_paths(dir, &home).unwrap();
            prompt = format!("{} {} ~/{}  ", exit_code, local, dir.display());
        } else {
            prompt = format!("{} {} {}  ", exit_code, local, dir.display());
        }

        return self.prompt_sigil(prompt);
    }

    pub fn builtin(&mut self, name: &String) -> Option<&fn(&mut Shell, Vec<String>) -> i32> {
        let mut name = name.as_str();
        if ALIASES.contains_key(&name) {
            name = ALIASES.get(&name).unwrap();
        }

        if self.builtins.contains_key(name) {
            return Some(self.builtins.get(name).unwrap());
        }

        return None;
    }

    pub fn exec<S: AsRef<str>>(&mut self, input: S) -> i32 {
        let parts = self.parse(input);
        if parts.is_none() {
            return 0;
        }

        let (operand, arguments) = parts.unwrap();
        return self.act(operand, arguments);
    }

    fn parse<S: AsRef<str>>(&mut self, input: S) -> Option<(String, Vec<String>)> {
        let mut parts: Vec<String> = input
            .as_ref()
            .split(' ')
            .filter(|p| !p.eq(&""))
            .map(|p| p.to_string())
            .collect();

        if parts.len() < 1 {
            return None;
        }

        return Some((parts.remove(0), parts));
    }

    // TODO: THIS FUNCTION SO MANY .unwrap()
    // FIXME: crashes on unknown command, check if it exists before calling Command::new on it
    fn act<S: AsRef<str>>(&mut self, operand: S, arguments: Vec<S>) -> i32 {
        let operand = operand.as_ref().to_string();
        let arguments = arguments.iter().map(|a| a.as_ref().to_string());

        if let Some(command) = self.builtin(&operand) {
            return command(self, arguments.collect());
        }

        // if !PathBuf::from(&operand).exists() {
        //     self.errln(format!("\"{}\" not found", operand)).ok();
        //     return 255;
        // }

        let mut child = Command::new(operand)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        // let stdin = child.stdin.take().expect("failed to get stdin");
        let mut stdout = child.stdout.take().expect("failed to get stdout");
        let mut stderr = child.stderr.take().expect("failed to get stderr");

        let mut buffer = [0u8; 4096];
        loop {
            let n = stdout.read(&mut buffer).unwrap();
            if n != 0 {
                self.write_buf(&buffer[..n]).ok();
            }

            let m = stderr.read(&mut buffer).unwrap();
            if m != 0 {
                self.err_buf(&buffer[..m]).ok();
            }

            if n == 0 && m == 0 {
                break;
            }
        }

        return child.wait().unwrap().code().unwrap_or(255);
    }
}
