mod generic_writer;
mod writer;
use std::fs::File;
use std::io::{stdout, Result, Stdout};

use writer::MyWriter;
fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abcd")?;

    let mut writer2: generic_writer::MyWriter<Stdout> = generic_writer::MyWriter::new(stdout());
    writer2.write_all(b"abcd")?;

    Ok(())
}
