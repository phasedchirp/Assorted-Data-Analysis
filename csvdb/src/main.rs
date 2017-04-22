use std::io::{self, Read, Write, ErrorKind};

fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
    where R: Read, W: Write
    {
        let mut buf = [0; 8000];
        let mut written = 0;
        loop {
            let len = match reader.read(&mut buf) {
                Ok(0)      => return Ok(written),
                Ok(len)    => len,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e)     => return Err(e)
            };
            writer.write_all(&buf[..len])?;
            written += len as u64
        }
}

fn main() {
    println!("Hello, world!");
}
