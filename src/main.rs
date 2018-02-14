extern crate glob;

use glob::glob;
use std::env;
use std::fs;

#[derive(Debug)]
struct File {
    path: String,
    size: u64,
}

#[derive(Debug)]
struct Group {
    paths: Vec<String>,
    size: u64,
}

impl Group {
    pub fn new() -> Group {
        let paths = Vec::new();
        Group { paths: paths, size: 0 }
    }
}

fn split_files(files: &mut Vec<File>, group_count: usize) -> Vec<Group> {
    let mut groups: Vec<Group> = Vec::with_capacity(group_count);
    for _ in 0..group_count {
        groups.push(Group::new());
    }

    files.sort_by_key(|file| file.size);

    while !files.is_empty() {
        let file = files.pop().unwrap();
        groups[0].size += file.size;
        groups[0].paths.push(file.path.clone());

        groups.sort_by_key(|group| group.size);
    }

    groups
}

fn get_files(glob_pattern: String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    for entry in glob(&glob_pattern).unwrap() {
        let path = entry.unwrap();
        let file = File {
            path: String::from(path.to_str().unwrap()),
            size: fs::metadata(path.as_path()).unwrap().len(),
        };
        files.push(file);
    }

    files
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let glob_pattern = args[1].clone();
    let group_count = args[2].parse::<usize>().unwrap();
    let group_index = args[3].parse::<usize>().unwrap();

    let mut files = get_files(glob_pattern);
    eprintln!("Total file count: {}", files.len());

    let groups = split_files(&mut files, group_count);
    let group = &groups[group_index];
    eprintln!("Group index: {}/{}", group_index, group_count - 1);
    eprintln!("Group file count: {}", &group.paths.len());
    eprintln!("Group size (bytes): {}", &group.size);

    println!("{}", &group.paths.join(" "));
}
