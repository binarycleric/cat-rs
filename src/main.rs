use std::env;
use std::path::Path;
use std::io::prelude::*;
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

fn main() {
    match env::args().nth(1) {
        Some(p) => {
            let path = Path::new(&p);
            let file = File::open(path);

            match file { 
                Ok(f) => {
                    println!("{}", file_contents(f));
                },
                Err(error) => {
                    println!("{}", error);
                    std::process::exit(1);
                }
            };
        },
        None => {
            println!("Usage: cat [path-to-file]");
            std::process::exit(1);
        }
    };
}
