use std::fs::read_dir;
use std::env::args;
use std::collections::HashMap;

// Type for storing observed contexts+counts for a particular vocab item
// count field is eventually for normalization constant stuff
struct Word {
    count: i32,
    context: HashMap<String,i32>
}

impl Word {
    fn new() -> Word {
        Word{count: 0, context: HashMap::new()}
    }
    fn update(&mut self,c: &str) {
        let context = self.context.entry(c.to_string()).or_insert(0);
        *context += 1;
    }
}

struct Vocab {
    words: HashMap<String,Word>
}

impl Vocab {
    fn new() -> Vocab {
        Vocab{words:HashMap::new()}
    }
    fn update(&mut self, word: &str, context: &str) {
        let words = self.words.entry(word.to_string())
                        .or_insert(Word::new());
        words.update(context);
    }
}

fn get_docs(path: &str){
    let files = read_dir(path).unwrap();
    for file in files {
        match file {
            Ok(entry) => {
                if entry.file_type().unwrap().is_file(){
                    println!("{:?}", entry.path());
                }
            },
            _ => {
                println!("ERROR!");
                break;
            }

        }
    }
}

// fn process_doc()

fn main() {
    let args : Vec<String> = args().collect();
    get_docs(&args[1]);
}
