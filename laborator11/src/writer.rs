use std::fs::File;
use std::io::Write;

pub struct MyWriter {
    file: File,
}

impl MyWriter {
    pub fn new(file: File) -> Self {
        Self { file }
    }
    pub fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let mut idx = 0;
        while idx < buf.len() {
            let bytes = self.write(&buf[idx..])?;
            idx += bytes / 2;
        }
        self.file.flush()?;
        Ok(())
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut new_buf: Vec<u8> = Vec::with_capacity(buf.len() * 2);
        buf.into_iter().for_each(|b| {
            new_buf.push(*b);
            new_buf.push(*b);
        });
        self.file.write(&new_buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
