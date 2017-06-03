#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate time;

use std::io::{Read,Write,stdin};
use std::fs::{OpenOptions,read_dir};
use std::env::args;
use std::collections::{HashMap,HashSet};

use time::now_utc;

#[derive(Debug,Serialize,Deserialize)]
struct Index {
    tags: HashMap<String,HashSet<String>>,
    words: HashMap<String,HashSet<String>>,
}

impl Index {
    fn from_file(path: &str) -> Index {
        let mut ind_file = OpenOptions::new().
                           create(true).
                           read(true).
                           write(true).
                           open(path).
                           unwrap();
        let mut ind_str = String::new();
        ind_file.read_to_string(&mut ind_str).unwrap();
        // if file was empty:
        if ind_str.is_empty() {
            ind_str = "tags = {}\nwords={}".to_string();
        }

        toml::from_str(&ind_str).unwrap()
    }

    fn to_file(&self, path: &str) {
        let mut ind_file = OpenOptions::new().
                            create(true).
                            write(true).
                            truncate(true).
                            open(path).
                            unwrap();

        let ind_ser = toml::to_string(&self).unwrap();
        match ind_file.write_all(&ind_ser.as_bytes()) {
            Ok(_) => println!("\nWrote updated index to file {}",
                               path.trim()),
            Err(e) => println!("\nEncountered an error: {:?}", e)
        }
    }

    fn update(&mut self,tags: &str, words: &str, label: &str) {
        for tag in tags.split(',') {
            let tagged = self.tags.entry(tag.trim().to_string()).or_insert(HashSet::new());
            (*tagged).insert(label.to_string());
        }

        for word in extract(words) {
            let instances = self.words.entry(word).or_insert(HashSet::new());
            (*instances).insert(label.to_string());
        }
    }
}

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

    fn to_file(&self, path: &str) {
        let mut entry_file = OpenOptions::new().
                            create_new(true).
                            write(true).
                            open(path).
                            unwrap();

        let entry_ser = toml::to_string(&self).unwrap();

        match entry_file.write_all(&entry_ser.as_bytes()) {
            Ok(_) => println!("\nWrote entry to file {}",path),
            Err(e) => println!("\nEncountered an error: {:?}", e)
        }
    }
}

fn extract(e: &str) -> Vec<String> {
    let mut vocab = e.split_whitespace().map(|s| s.trim().to_string()).collect::<Vec<String>>();
    vocab.sort();
    vocab.dedup();
    vocab
}

fn write_new(journal_dir: &str) {
    let mut inputs = String::new();
    let timestamp = now_utc().ctime().to_string();
    println!("{}", timestamp);
    println!("------------------------");

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
    let mut index = Index::from_file(&format!("{}/.index.toml",journal_dir.trim()));

    index.update(&tags, &inputs, &timestamp);

    let entry = Entry::new(timestamp,tags, inputs);

    entry.to_file(&format!("{}/{}",journal_dir.trim(),entry.timestamp));
    index.to_file(&format!("{}/.index.toml",journal_dir.trim()));
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
