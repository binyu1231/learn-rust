
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};


fn main() {
    let f_cont = read_username_from_file().unwrap();
    println!("file content: {:?}", f_cont);
}

// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }


// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// use std::io;
// use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn _create_file() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err)
            }
        },
    };

    println!("hello file {:?}", f);
}
