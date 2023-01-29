#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io::*;
mod Common;
mod Day1;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "1:1" => Day1::part_1(),
        "1:2" => Day1::part_2(),
        _ => println!("Invalid input"),
    }
}
