use std::fs::File;
use std::io::Read;

pub fn read_file() -> Vec<u8> { 
    let mut file = File::open("src/Day1/input.txt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    return contents;
}