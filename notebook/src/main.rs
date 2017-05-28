extern crate time;

use std::io::{Write,stdin};
use std::fs::{OpenOptions,read_dir};
use std::env::args;
// use std::time::{SystemTime};
use time::now_utc;

fn write_new(journal_dir: &str) {
    let mut inputs = String::new();
    let timestamp = now_utc();
    println!("{}", timestamp.ctime());
    // let mut journal_dir = String::new();
    // println!("Specify a journal directory:");
    // stdin().read_line(&mut journal_dir).unwrap();

    println!("");

    let mut entry_file = OpenOptions::new().
                        create_new(true).
                        write(true).
                        open(format!("{}/{}",journal_dir.trim(),timestamp.ctime())).
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
    match entry_file.write_all(&inputs.as_bytes()) {
        Ok(_) => println!("\nWrote entry to file {}",
                           format!("{}/{}",journal_dir.trim(),timestamp.ctime())),
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
