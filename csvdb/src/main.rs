extern crate toml;

use toml::Value;

use std::io::{self, Read, Write, ErrorKind,BufReader,BufRead};
use std::fs::{File,OpenOptions};
use std::env::args;

// function for copying contents of reader to writer
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

// eventually function to convert csv lines to SQL insert statements
fn lines_to_queries<R: BufRead>(f: &mut R) -> () {
    for line in f.lines() {
        println!("{:?}", line);
    }
}

fn main() {
    let args : Vec<String> = args().collect();
    let mut c_string = String::new();
    if args.len() > 3 {
        let mut cfg = File::open(&args[3]).expect("couldn't open config file");
        cfg.read_to_string(&mut c_string).unwrap();
    }

    let config = c_string.parse::<Value>().unwrap();

    println!("{:?}",config["placeholder"].as_str());

    let mut f_in = File::open(&args[1]).expect("couldn't open input file");
    let mut f_out = OpenOptions::new().write(true).create_new(true).open(&args[2]).expect("couldn't open output file");
    let bytes = copy(&mut f_in, &mut f_out);
    let mut b_in = BufReader::new(File::open(&args[1]).unwrap());
    lines_to_queries(&mut b_in);
    println!("Copied {:?} bytes?",bytes);
}
