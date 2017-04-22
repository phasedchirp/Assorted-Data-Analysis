use std::io::{self, Read, Write, ErrorKind};
use std::fs::{File,OpenOptions};
use std::env::args;

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
    let args : Vec<String> = args().collect();
    let mut f_in = File::open(&args[1]).expect("couldn't open input file");
    let mut f_out = OpenOptions::new().write(true).create_new(true).open(&args[2]).expect("couldn't open output file");
    let bytes = copy(&mut f_in, &mut f_out);
    println!("Copied {:?} bytes?",bytes);
}
