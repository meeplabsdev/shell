use crate::error::Error;
use std::io::{BufRead, BufReader, Read, Write};

pub struct IoFace {
    stdin: Box<dyn Read>,
    stdout: Box<dyn Write>,
    stderr: Box<dyn Write>,
}

impl IoFace {
    pub fn new<I: Read + 'static, O: Write + 'static, E: Write + 'static>(
        stdin: I,
        stdout: O,
        stderr: E,
    ) -> Self {
        Self {
            stdin: Box::new(stdin),
            stdout: Box::new(stdout),
            stderr: Box::new(stderr),
        }
    }

    pub fn read_buf(&mut self, buffer: &mut String) -> Result<(), Error> {
        let mut reader = BufReader::new(self.stdin.by_ref());
        reader.read_line(buffer)?;

        return Ok(());
    }

    pub fn read(&mut self) -> Result<String, Error> {
        let mut buffer = String::new();
        self.read_buf(&mut buffer)?;

        return Ok(buffer.trim().to_string());
    }

    pub fn write_buf(&mut self, buffer: &[u8]) -> Result<(), Error> {
        self.stdout.write_all(buffer)?;
        self.stdout.flush()?;

        return Ok(());
    }

    pub fn write(&mut self, content: String) -> Result<(), Error> {
        self.write_buf(content.as_bytes())
    }

    pub fn writeln(&mut self, content: String) -> Result<(), Error> {
        let mut content = content;
        if !content.ends_with('\n') {
            content.push('\n');
        }

        return self.write(content);
    }

    pub fn err_buf(&mut self, buffer: &[u8]) -> Result<(), Error> {
        self.stderr.write_all(buffer)?;
        self.stderr.flush()?;

        return Ok(());
    }

    pub fn err(&mut self, content: String) -> Result<(), Error> {
        self.err_buf(content.as_bytes())
    }

    pub fn errln(&mut self, content: String) -> Result<(), Error> {
        let mut content = content;
        if !content.ends_with('\n') {
            content.push('\n');
        }

        return self.err(content);
    }
}
