#![allow(non_snake_case)]
#![allow(dead_code)]

use std::io::*;
mod Day1;
mod Common;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    println!("You typed: {}", input);
           match input.trim() {
            "1_1" => Day1::part_1(),
            "1_2" => Day1::part_2(),
            _ => println!("Invalid input"),
        }

    println!("Hello, world!");
    Day1::part_1();
    Common::read_file();
}