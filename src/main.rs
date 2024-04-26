use std::{env, fs};

struct File {
    path: String,
    file_name: String,
}

impl File {
    fn new(path: String, file_name: String) -> File {
        File {
            path,
            file_name,
        }
    }
}

fn main() {
    let path = match env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Couldn't get the current directory: {}", e),
    };
    println!("The current directory is {}", path.display());

    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(e) => panic!("Couldn't read the current directory: {}", e),
    };

    let files: Vec<String> = paths.map(|entry| {
        let entry = entry.unwrap();
         match entry.file_name().to_str() {
            Some(file_name) => file_name.to_string(),
            None => panic!("Couldn't convert the file name to a string"),
        }
    }).collect();

    for file in files {
        println!("{}", file);
    }
}
