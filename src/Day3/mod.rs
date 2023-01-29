

use crate::Common::{parse_file};
use std::f64;



fn eval_to_bits(length: Vec<String>, sum : Vec<u32>) -> Vec<i32>{
    let len = length.len();
    let comp = (len as f64 / 2.0).trunc() as u32;
    let mut arr: Vec<i32>  = Vec::new();
    for (_i, &val) in sum.iter().enumerate() {
         if val > comp  {
            arr.push(1);
         }
         else {
            arr.push(0);
         }
    }
    arr
}

fn accumulate_vertical(bits_per_line: Vec<Vec<u32>>) -> Vec<u32>{
    let mut sums = vec![0; bits_per_line[0].len()];
    for row in bits_per_line {
        for (i, &val) in row.iter().enumerate() {
            sums[i] += val;
        }
    }
    sums
}
fn accumulate_bits(arr: Vec<i32>) -> u32{
    let mut result = 0;
    for (i, &x) in arr.iter().enumerate() {
        result |= (x as u32) << i;
    }
    println!("orig: {:?} {:b}",arr, result);
    result
}

fn flip_bits (arr: Vec<i32>)->Vec<i32>{
    let mut _arr = arr.clone();
    for (i, &x) in arr.iter().enumerate() {
        if x == 1 {
            _arr[i] = 0;
        }
        else {
            _arr[i] = 1;
        }
    }
    _arr

}
fn string_to_bits( _lines: Vec<String>) -> Vec<Vec<u32>>{

    let mut bits_per_line: Vec<Vec<u32>> = Vec::new();
    for line in _lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut bits: Vec<u32> = Vec::new();
        
        for char in chars {
            bits.push(char.to_digit(2).unwrap());
        }
        bits_per_line.push(bits);
        
    }
    bits_per_line
}
fn accumulate_to_num(gamma_arr: Vec<i32>)->i32{
    let mut gamma = 0;
    for i in 0..gamma_arr.len() {
        gamma += gamma_arr[i] * 2_i32.pow((gamma_arr.len()  - i - 1) as u32);
    }
    gamma
}

pub fn part_1() { 

    let _lines = parse_file("src/Day3/input.txt");
    let length = _lines.clone();
    
    let bits_per_line = string_to_bits( _lines);
    
    let sum = accumulate_vertical(bits_per_line);
    let gamma_arr = eval_to_bits(length, sum);
    let epsilon_arr = flip_bits(gamma_arr.clone());
   
    let gamma = accumulate_to_num(gamma_arr);
    let epsilon = accumulate_to_num(epsilon_arr);
    
    
    let _res =  epsilon * gamma ; 
    println!("Part 1: {}", _res);
}

pub fn part_2() {

    let _lines = parse_file("src/Day3/input.txt");
    let _res = 0;
    println!("Part 2: {}", _res);
}