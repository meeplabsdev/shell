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
    sync::{Arc, Mutex},
};

static ALIASES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    ":" => "noop",
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
}
