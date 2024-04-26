use std::{env, fs};
use std::fs::DirEntry;

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Couldn't get the current directory: {}", e),
    };

    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(e) => panic!("Couldn't read the current directory: {}", e),
    };

    let files: Vec<DirEntry> = paths.map(|entry| entry.unwrap())
        .collect();

    for file in files.iter() {
        let file_name = file.file_name().into_string().unwrap();
        print!("{} ", file_name);
    }
    println!();
}
