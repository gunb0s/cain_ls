use std::{env, fs};
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;
use colored::Colorize;

fn main() {
    let path: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Couldn't get the current directory: {}", e),
    };

    let paths: ReadDir = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(e) => panic!("Couldn't read the current directory: {}", e),
    };

    let files: Vec<DirEntry> = paths
        .map(|entry| entry.unwrap())
        .collect();

    let max_length = files
        .iter()
        .map(|entry| entry.file_name().len())
        .max()
        .unwrap();

    files.iter().for_each(|entry| {
        // let file_name = entry.file_name().to_str().unwrap();
        let metadata = entry.metadata().unwrap();
        let binding = entry.file_name();
        let file_name = match binding.to_str() {
            Some(file_name) => {
                if metadata.is_dir() {
                    file_name.to_string().blue()
                } else {
                    file_name.to_string().bold()
                }
            },
            None => panic!("Couldn't convert the file name to a string"),
        };
        let file_name = format!("{:width$}", file_name, width = max_length);
        print!("{file_name} ");
    });
    println!();
}
