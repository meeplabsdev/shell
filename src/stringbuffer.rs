use std::{
    io::{Read, Write},
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct StringBuffer {
    content: Arc<Mutex<String>>,
}

impl Read for StringBuffer {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        return buf.write(&self.content.lock().unwrap().as_bytes());
    }
}

impl Write for StringBuffer {
    fn write(&mut self, mut buf: &[u8]) -> std::io::Result<usize> {
        let mut data = String::new();
        let result = buf.read_to_string(&mut data);
        self.content.lock().unwrap().push_str(&data);
        return result;
    }

    fn flush(&mut self) -> std::io::Result<()> {
        return Ok(());
    }
}

impl Into<String> for StringBuffer {
    fn into(self) -> String {
        return self.content.lock().unwrap().to_string();
    }
}

impl StringBuffer {
    pub fn new() -> Self {
        Self {
            content: Arc::new(Mutex::new(String::new())),
        }
    }
}
