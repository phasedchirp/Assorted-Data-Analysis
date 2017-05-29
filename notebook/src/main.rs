#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate time;

use std::io::{Write,stdin};
use std::fs::{OpenOptions,read_dir};
use std::env::args;
use time::now_utc;

#[derive(Debug,Serialize)]
struct Entry {
    timestamp: String,
    tags: Vec<String>,
    content: String,
}

impl Entry {
    fn new(ts: String, tags: String, c: String) -> Entry {
        Entry{
            timestamp: ts,
            tags: tags.split(',').map(|s| s.trim().to_string()).collect(),
            content: c
        }
    }
}

fn write_new(journal_dir: &str) {
    let mut inputs = String::new();
    let timestamp = now_utc().ctime().to_string();
    println!("{}", timestamp);
    println!("------------------------");

    let mut entry_file = OpenOptions::new().
                        create_new(true).
                        write(true).
                        open(format!("{}/{}.toml",journal_dir.trim(),timestamp)).
                        unwrap();

    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                if &buffer == "goodbye notebook\n" {
                    break;
                } else {
                    inputs += &buffer;
                }
            },
            Err(e) => println!("{:?}",e)
        }
    }

    let mut tags = String::new();
    println!("Tags (comma-separated):");
    match stdin().read_line(&mut tags) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e)
    }

    let entry = Entry::new(timestamp,tags, inputs);

    let entry_ser = toml::to_string(&entry).unwrap();

    match entry_file.write_all(&entry_ser.as_bytes()) {
        Ok(_) => println!("\nWrote entry to file {}",
                           format!("{}/{}",journal_dir.trim(),entry.timestamp)),
        Err(e) => println!("\nEncountered an error: {:?}", e)
    }
}

fn list_entries(journal_dir: &str) {
    println!("The following entries exist:");
    let es = read_dir(journal_dir).unwrap();
    for e in es {
        println!("{:?}", e.unwrap().path());
    }
}

fn main() {
    let mut inputs = args();
    if let Some(s) = inputs.nth(1) {
        match &*s {
            "--new" => write_new(&inputs.nth(0).unwrap().trim()),
            "--list" => list_entries(&inputs.nth(0).unwrap().trim()),
            _ => println!("Please specify a valid mode.\nThese include:\n--new\n--list")
        }
    }

}
