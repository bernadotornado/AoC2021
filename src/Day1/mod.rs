use crate::Common::{parse_file};
use itertools::{Itertools};

pub fn part_1() { 

    let _lines = parse_file("src/Day1/input.txt");
    let mut _sum = 0;
    let mut _last = 0;
    for line in _lines {
        let num = line.trim().parse::<i32>().unwrap();
        if _last > 0 && num > _last {
            _sum += 1;
        }
        _last = num;        
    }

    println!("Part 1: {}", _sum);
}

pub fn part_2(){

    let _lines = parse_file("src/Day1/input.txt");
   
    let mut _sum = 0;
    let mut _last = 0;
    
    for (a,b,c) in _lines.iter().tuple_windows() {
        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        let c = c.parse::<i32>().unwrap();
        let window_sum = a + b + c;
        if _last > 0 && window_sum > _last {
            _sum += 1;
        }
        _last = window_sum;
    }

    println!("Part 2: {}", _sum);
}