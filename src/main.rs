use std::env;
use std::fs;
use std::io::*;

#[derive(Debug)]
struct DirEntryContainer {
    name: String,
}

impl DirEntryContainer {
    pub fn new(name: &str) -> DirEntryContainer {
        DirEntryContainer {
            name: String::from(name)
        }
    }

    pub fn to_string(&self) -> &String {
        &self.name
    }
}

fn main() {
    println!("Hello, Xebia!");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let path = ".";
    let resultDir = fs::read_dir(path);

    match resultDir {
        Ok(value) => toto(value),
        Err(e) => println!("Error reading the path: {}", path)
    }
}

fn toto(value: fs::ReadDir) {
    for path in value {
        println!("Name: {}", path.unwrap().path().display())
    }
}
