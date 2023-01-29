use std::fs::File;
use std::io::Read;
// use std::path::{Path, self};
// use std::string;
use std::io::{BufReader, BufRead};

pub fn parse_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = vec![];

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    return lines; 
}



pub fn read_file(path: String) -> Vec<u8> { 
    let mut file = File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    return contents; 
}

pub fn read_file_to_reader(path: String) -> std::io::BufReader<std::fs::File> { 
    let file = File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    return reader;
}