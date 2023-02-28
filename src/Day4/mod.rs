use crate::Common::parse_file;
use std::{f64, vec};

type Board = Vec<Vec<i32>>;
trait Printable {
    fn print(&self);
}
impl Printable for Board {
    fn print(&self) {
        for row in self {
            for col in row {
                print!("{} ", col);
            }
            println!("");
        }
    }
}


fn parse_boards (lines: &Vec<String>) -> Vec<Board> {
    let mut board: Board = vec![vec![]];
    let mut boards: Vec<Board> = vec![];
    for (i, s) in lines.iter().enumerate() {
        if i == 0 || i == 1 {
            continue;
        }
        if !s.is_empty() {
            board.push(s
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim_end()
                .parse::<i32>()
                .unwrap())
                .collect::<Vec<i32>>());
        }
        else {
            boards.push(board);
            board = vec![vec![]];
        }
    }
    return boards;
}

fn mark_board (boards: Vec<Board>, drawn_numbers: Vec<i32>) -> Vec<Vec<i32>>{
    let mut containing:Vec<Vec<i32>> = vec![vec![]];
    for board in boards {
        board.print();
        
        

        for (i, row) in board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if drawn_numbers.contains(&col) {
                    containing[i].push(1);
                }
                else {
                    containing[i].push(0);
                }
            }
        }
    }
    return containing;
}



pub fn part_1(){
    
    let lines = parse_file("src/Day4/input.txt");
    let drawn_numbers = lines[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Drawn numbers: {:?}", drawn_numbers);
    let boards: Vec<Board> = parse_boards(&lines);

    let mut numbers_up_to_current:Vec<i32> = vec![];
    for number in drawn_numbers{
        numbers_up_to_current.push(number);
        let containing = mark_board(boards.clone(), numbers_up_to_current.clone());
        containing.print();
    }

    
    
}

pub fn part_2(){

}