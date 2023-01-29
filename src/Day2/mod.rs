use crate::Common::{parse_file};

pub fn part_1() { 

    let _lines = parse_file("src/Day2/input.txt");
    let mut _horizontal = 0;
    let mut _depth = 0;
    for line in _lines {
        let tokens = line.trim().split_whitespace().collect::<Vec<&str>>();
        match (tokens.get(0), tokens.get(1)){
            (Some(&"forward"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _horizontal += num;
                } 
            },
            (Some(&"up"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _depth -= num;
                } 
            },
            (Some(&"down"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _depth += num;
                } 
            },
            _ => println!("Invalid input"),
        }
  }
    let _res = _horizontal * _depth;
    
    println!("Part 1: {}", _res);
}

pub fn part_2() {

    let _lines = parse_file("src/Day2/input.txt");
    let mut _horizontal = 0;
    let mut _depth = 0;
    let mut _aim = 0;
    for line in _lines {
        let tokens = line.trim().split_whitespace().collect::<Vec<&str>>();
        match (tokens.get(0), tokens.get(1)){
            (Some(&"forward"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _horizontal += num;
                    _depth += _aim*num;
                } 
            },
            (Some(&"up"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _aim -= num;
                } 
            },
            (Some(&"down"), Some(amount)) => {
                if let Ok(num) = amount.parse::<i32>() {
                    _aim += num;
                } 
            },
            _ => println!("Invalid input"),
        }
  }
    let _res = _horizontal * _depth;
    
    println!("Part 1: {}", _res);
}