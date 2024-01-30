use std::{io::{stdin, Stdin}, num::ParseIntError};

#[derive(Debug)]
enum Turn {
    X,
    O,
}

#[derive(Debug)]
enum GameResult {
    WinX,
    WinO,
    Tie,
}

fn get_board() -> [[Option<char>; 3]; 3] {
    [[None; 3]; 3]
}

pub fn game_loop() {
    let mut board = get_board();
    let mut turn = Turn::X;
    loop {
        println!("Turn {:?}", turn);
        print_board(&board);
        loop {
            let (x, y) = get_position();
            let cell = board[x][y];
            match cell {
                None => {
                    board[x][y] = match turn {
                        Turn::X => Some('X'),
                        Turn::O => Some('O'),
                    };
                    break;
                },
                Some(_) => {
                    println!("Wrong position, try again");
                    continue;
                }
            }
        }
        match check_winner(&board) {
            None => (),
            Some(res) => {
                println!("The result is {:?}", res);
                print_board(&board);
                break;
            },
        };
        match turn {
            Turn::X => turn = Turn::O,
            Turn::O => turn = Turn::X,
        }
    }
}


fn check_winner(board: &[[Option<char>; 3]; 3]) -> Option<GameResult> {
    // checking rows
    for i in 0..3 {
        if board[i][0] == board[i][1] && board[i][2] == board[i][1] {
            match board[i][0] {
                None => continue,
                Some(c) => {
                    match c {
                        'X' => return Some(GameResult::WinX),
                        'O' => return Some(GameResult::WinO),
                        _ => unreachable!()
                    }
                }
            }
        }
        if board[0][i] == board[1][i] && board[2][i] == board[1][i] {
            match board[0][i] {
                None => continue,
                Some(c) => {
                    match c {
                        'X' => return Some(GameResult::WinX),
                        'O' => return Some(GameResult::WinO),
                        _ => {
                            println!("{c}");
                            unreachable!();
                        } 
                    }
                }
            }
        }
    }
    // checking diagonals
    if (board[0][0] == board[1][1] && board[2][2] == board[1][1]) ||
    (board[0][2] == board[1][1] && board[1][1] == board[2][0]) {
        match board[1][1] {
            None => (),
            Some(c) => {
                match c {
                    'X' => return Some(GameResult::WinX),
                    'O' => return Some(GameResult::WinO),
                    _ => unreachable!()
                }
            }
        }
    }
    // checking for tie
    let mut count = 0;
    for i in 0..3 {
        for j in 0..3 {
            match board[i][j] {
                None => break,
                Some(_) => { count += 1; }
            }
        }
    }
    if count == 9 {
        return Some(GameResult::Tie)
    } else {
        None
    }
}

fn print_board(board: &[[Option<char>; 3]; 3]) {
    println!("         1 2 3 ");
    for (idx_row, &row) in board.iter().enumerate() {
        for (idx, &c) in row.iter().enumerate() {
            match c {
                None => {
                    if idx == 0 {
                        print!("      {} | ", idx_row+1)
                    } else if idx == 1 {
                        print!("| |")
                    } else {
                        print!(" |")
                    }
                },
                Some(c) => {
                    if idx == 0 {
                        print!("      {} |{c}", idx_row+1)
                    } else if idx == 1 {
                        print!("|{c}|")
                    } else {
                        print!("{c}|")
                    }
                },
            }
        }
        println!()
    }
}

fn get_position() -> (usize, usize) {
    let stdin = stdin();
    let mut res: Vec<Result<usize, ParseIntError>>;
    loop {
        println!("Enter the (x, y) (in range 1..3) for X, splitted with space");
        let mut input = String::new();
        stdin.read_line(&mut input);
        res = input
            .split_whitespace()
            .map(|x| {
                x.parse::<usize>()
            })
            .collect();
        if res.len() != 2 {
            println!("Wrong position, try again");
            continue
        }
        match (&res[0], &res[1]) {
            (Ok(val1), Ok(val2)) => {
                if (1..=3).contains(val1) && (1..=3).contains(val2) {
                    return (*val1 - 1, *val2 - 1)
                } else {
                    println!("Wrong position, try again");
                    continue
                }
            },
            _ => { 
                println!("Wrong position, try again");
                continue
            },
        }
    }
}
