use std::io::{Read, Write, Result, Error, ErrorKind};


pub struct ReadIOError {}

impl Read for ReadIOError
{
    fn read(&mut self, _: &mut [u8]) -> Result<usize>
    {
        Err(Error::new(ErrorKind::Other, "Mock Read Error"))
    }
}


pub struct WriteIOError {}

impl Write for WriteIOError
{
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize>
    {
        Err(Error::new(ErrorKind::Other, "Mock Write Error"))
    }

    fn flush(&mut self) -> Result<()>
    {
        Ok(())
    }
}