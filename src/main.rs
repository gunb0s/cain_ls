use std::{env, fs};
use std::fs::{DirEntry, ReadDir};
use std::os::unix::fs::MetadataExt;
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;

use chrono::{Local, LocalResult, TimeZone};
use colored::Colorize;

use cain_ls::config::Config;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            std::process::exit(1);
        },
    };

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
        .filter(|entry| {
            if config.all_flag {
                true
            } else {
                entry.file_name().as_bytes()[0] != b'.'
            }
        }).collect();

    let max_length = files
        .iter()
        .map(|entry| entry.file_name().len())
        .max()
        .unwrap();

    files.iter().for_each(|entry| {
        let metadata = entry.metadata().unwrap();

        let mode = metadata.mode();

        let nlinks = metadata.nlink();

        let uid = metadata.uid();
        let gid = metadata.gid();

        let size = metadata.len();

        let modified = metadata.modified().unwrap();
        let datetime = match Local.timestamp_opt(
            modified
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as i64
            , 0
        ) {
            LocalResult::Single(datetime) => datetime.format("%b %d %H:%M").to_string(),
            LocalResult::None => panic!("Couldn't convert the modified time to a datetime"),
            LocalResult::Ambiguous(_, _) => panic!("Ambiguous datetime"),
        };

        // let file_name = entry.file_name().to_str().unwrap();
        let binding = entry.file_name();
        let file_name = match binding.to_str() {
            Some(file_name) => {
                if metadata.is_dir() {
                    file_name.to_string().bright_blue()
                } else {
                    file_name.to_string().bold()
                }
            },
            None => panic!("Couldn't convert the file name to a string"),
        };

        if config.show_detail_flag {
            println!("{} {} {} {} {} {} {}",
                mode_to_string(mode),
                nlinks,
                uid,
                gid,
                size,
                datetime,
                file_name
            );
        } else {
            let file_name = format!("{:width$}", file_name, width = max_length);
            print!("{file_name} ");
        }
    });

    println!();
}

fn mode_to_string(mode: u32) -> String {
    let mut result = String::with_capacity(10);

    // 파일 유형 (디렉토리, 일반 파일 등)
    result.push(if mode & 0o40000 == 0o40000 { 'd' } else { '-' });

    // 사용자 권한
    result.push(if mode & 0o400 == 0o400 { 'r' } else { '-' });
    result.push(if mode & 0o200 == 0o200 { 'w' } else { '-' });
    result.push(if mode & 0o100 == 0o100 { 'x' } else { '-' });

    // 그룹 권한
    result.push(if mode & 0o040 == 0o040 { 'r' } else { '-' });
    result.push(if mode & 0o020 == 0o020 { 'w' } else { '-' });
    result.push(if mode & 0o010 == 0o010 { 'x' } else { '-' });

    // 기타 사용자 권한
    result.push(if mode & 0o004 == 0o004 { 'r' } else { '-' });
    result.push(if mode & 0o002 == 0o002 { 'w' } else { '-' });
    result.push(if mode & 0o001 == 0o001 { 'x' } else { '-' });

    result
}
