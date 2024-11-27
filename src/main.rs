use std::io;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use rand::Rng;
const MAX : usize = 30;



fn PrintState(mat: &[[i32; MAX]; MAX]) {
    for i in 1..MAX {
        for j in 1..MAX {
            print!("{} ", mat[i][j]);
        }
        println!();
    }
    Command::new("cls");
    io::stdout().flush().unwrap();
    print!("\x1B[2J\x1B[1;1H");
}

fn GenerateBoard() -> [[i32; MAX]; MAX] {
    let mut mat : [[i32; MAX]; MAX] = [[0; MAX]; MAX];
    for i in 0..MAX {
        for j in 0..MAX {
            mat[i][j] = rand::thread_rng().gen_range(0..=1);
        }
    }
    return mat;
}

fn CalculateNext(atual : [[i32; MAX]; MAX]) -> [[i32; MAX]; MAX] {
    
}



fn main() {
    let mut i : usize = 0;
    let mut j : usize = 0;
    let mut board = GenerateBoard();
    PrintState(&board);


}
