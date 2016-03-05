/*
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("Cargo.toml");
    let mut s = String::new();

    match File::open("Cargo.toml") {
        Ok(value) => {
            value.read_to_string(&mut s)
        },
        Err(..) => {

        },
    }

    // f.read_line(&mut s)

}
*/

use std::io::prelude::*;
use std::fs::File;

fn cat_contents(mut f: File) {
    let mut contents = String::new();

    match f.read_to_string(&mut contents) {
        Ok(_) => { },
        Err(error) => {
            panic!("{}", error);
        },
    }
    print!("{}", contents);
}

fn main() {
    let file = File::open("Cargo.toml"); 

    match file { 
        Ok(f) => {
            cat_contents(f);
        },
        Err(error) => {
            panic!("{}", error); 
        }
    };
}
