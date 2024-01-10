use std::io::Write;

pub struct MyWriter<T>
where
    T: Write,
{
    file: T,
}

impl<T> MyWriter<T>
where
    T: Write,
{
    pub fn new(file: T) -> Self {
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

impl<T> Write for MyWriter<T>
where
    T: Write,
{
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
