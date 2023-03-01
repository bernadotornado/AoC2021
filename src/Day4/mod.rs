use crate::Common::parse_file;
use std::vec;

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
            println!(" ");
        }
    }
}


fn parse_boards (lines: &Vec<String>) -> Vec<Board> {
    let mut board: Board = vec![];
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
            // println!("board: {:?}", board);
            // if board.len()>0 {
                boards.push(board);
                board = vec![];
            // }
            
        }
    }
    return boards;
}

fn mark_board (board: Board, drawn_numbers: Vec<i32>) -> Vec<Vec<i32>>{
    let mut marked_board: Board = vec![];
    // for board in boards {
    //     board.print();

        for (i, row) in board.iter().enumerate() {
            println!("outer: i {}", i);
            println!("row: {:?}", row);
            let mut marked_row: Vec<i32> = vec![];
            for (j, col) in row.iter().enumerate() {
                println!("inner i:{} j:{}", i, j);
                if drawn_numbers.contains(&col) {
                    marked_row.push(1);
                }
                else {
                    marked_row.push(0);
                }
            }
            marked_board.push(marked_row);
        }
    // }
    return marked_board;
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
        let n = boards.clone();
        let mut marked_boards: Vec<Board> = vec![];
        for board in n {
            marked_boards.push(mark_board(board, numbers_up_to_current.clone()));
        }

        // let containing = mark_board(, numbers_up_to_current.clone());
        // containing.print();
    }
}

pub fn part_2(){

}