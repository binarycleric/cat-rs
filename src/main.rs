use std::env;
use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

fn file_contents(mut f: File) -> String {
    let mut contents = String::new();

    match f.read_to_string(&mut contents) {
        Ok(_) => { },
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        },
    }
   
    return contents;
}

fn from_stdin() -> String {
    let mut buffer = String::new();
    
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            return buffer; 
        },
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        },
    }
}

fn from_file(p: String) -> String {
    let path = Path::new(&p);
    let file = File::open(path);

    match file { 
        Ok(f) => {
            return file_contents(f);
        },
        Err(error) => {
            println!("{}", error);
            std::process::exit(1);
        }
    };
}

fn main() {
    match env::args().nth(1) {
        Some(p) => {
            println!("{}", from_file(p));
        },
        None => {
            println!("{}", from_stdin()); 
        }
    };
}
