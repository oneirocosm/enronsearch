use std::fs::File;
use std::error;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};

// easier way of navigating file system
use walkdir::WalkDir;

mod trie;
use crate::trie::Trie;

// hard-coding this value for now.  It should be user configurable in an application,
// but that is not the foucs of this exercise
const DIRECTORY: &str = r"..\maildir";

fn main() -> Result<(), Box<dyn error::Error>> {
    // create data structure to store words and locations
    let mut data = Trie::new();

    // read all of the data into the data structure
    for file in WalkDir::new(DIRECTORY) {
        let temp = file?;
        let file_path = temp.path();
        let reader: BufReader<File>;
        if let Ok(contents) = File::open(file_path.clone()){
            reader = BufReader::new(contents);
        } else {
            continue
        }

        for line in reader.lines() {
            for word in line?.split_whitespace() {
                data.insert(String::from(word), PathBuf::from(file_path))
            }
        }
    }

    // search for our query (currently limited to single words)
    let out = data.search(String::from("app"));
    for fname in out {
        println!("{:?}", fname);
    }
    Ok(())
}
