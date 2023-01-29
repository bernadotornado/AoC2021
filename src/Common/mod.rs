use std::fs::File;
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
