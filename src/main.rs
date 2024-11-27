use std::io;
use std::io::Write;
use std::process::Command;
use std::{thread, time};
use rand::Rng;
use std::env;
const MAX : usize = 80;

fn PrintState(mat: &[[i32; MAX]; MAX]) {
    //println!("-----------------------------------------------------------------------------------------------------------------------------");

    for i in 0..mat.len() {
        for j in 0..MAX {
            if(mat[i][j] == 1) {
                print!("{} ", mat[i][j]);
            } else {
                print!(" ");
                //print!("0 ");
            }
        }
        println!();
    }
    //println!("-----------------------------------------------------------------------------------------------------------------------------");
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


fn CheckRules(nCount : i32, currentCell : &mut i32) -> (){
    // *currentCell = nCount;
    // return;
    if(*currentCell == 0 && nCount == 3) {
        *currentCell = 1;
        return;
    }

    if(*currentCell == 1) {
        if(nCount == 2 || nCount == 3) {
            *currentCell = 1;
            return;
        }
        *currentCell = 0;
        return;
    }
}


fn CalculateNext(atual : &[[i32; MAX]; MAX]) -> [[i32; MAX]; MAX] {
    let mut new: [[i32; MAX]; MAX] = *atual;
    for i in 0..MAX {
        for j in 0..MAX {
            let mut nCount: i32 = 0;
            //-------------------------------casos de beirada---------------------------------------
            if(i == 0 && j == 0) {
                nCount += atual[i+1][j];
                nCount += atual[i+1][j+1];
                nCount += atual[i][j+1];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(i == MAX-1 && j == MAX-1) {
                nCount += atual[i-1][j];
                nCount += atual[i-1][j-1];
                nCount += atual[i][j-1];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(i == 0 && j == MAX-1) {
                nCount += atual[i][j-1];
                nCount += atual[i+1][j-1];
                nCount += atual[i+1][j];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(i == MAX-1 && j == 0) {
                nCount += atual[i-1][j];
                nCount += atual[i-1][j+1];
                nCount += atual[i][j+1];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(i == 0) {
                nCount += atual[i][j-1];
                nCount += atual[i+1][j-1];
                nCount += atual[i+1][j];
                nCount += atual[i+1][j+1];
                nCount += atual[i][j+1];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(i == MAX-1) {
                nCount += atual[i][j-1];
                nCount += atual[i-1][j-1];
                nCount += atual[i-1][j];
                nCount += atual[i-1][j+1];
                nCount += atual[i][j+1];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(j == 0) {
                nCount += atual[i-1][j];
                nCount += atual[i-1][j+1];
                nCount += atual[i][j+1];
                nCount += atual[i+1][j+1];
                nCount += atual[i+1][j];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            if(j == MAX-1) {
                nCount += atual[i-1][j];
                nCount += atual[i-1][j-1];
                nCount += atual[i][j-1];
                nCount += atual[i-1][j-1];
                nCount += atual[i+1][j];
                CheckRules(nCount, &mut new[i][j]);
                continue;
            }

            //--------------------------------------------------------------------------------------

            //posicao qlqr:
            nCount += atual[i-1][j-1]; //diagonal cima esquerda
            nCount += atual[i-1][j];   //pra cima
            nCount += atual[i-1][j+1]; //diagonal cima direita
            nCount += atual[i][j-1];   // esquerda
            nCount += atual[i][j+1];   // direita
            nCount += atual[i+1][j-1]; //diagonal baixo esquerda
            nCount += atual[i+1][j];   //pra baixo
            nCount += atual[i+1][j+1]; //diagonal baixo direita
            CheckRules(nCount, &mut new[i][j]);
        }
    }
    new
}


fn main() {
    let mut delay = 300;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let lido = (&args[1]).parse::<i32>().unwrap();
        delay = lido;
        println!("Delay entre atualizações definido: {}", delay);
    } else {
        println!("Delay não inserido como argumento, definindo para o padrão: 300")
    }
    thread::sleep(time::Duration::from_millis(1000));
    let mut board = GenerateBoard();
    PrintState(&board);
    loop {
        if delay == 0 {
            break;
        }
        board = CalculateNext(&board);
        PrintState(&board);
        thread::sleep(time::Duration::from_millis(delay.try_into().unwrap()));
        //clearscreen::clear().expect("failed to clear screen");

    }

}
