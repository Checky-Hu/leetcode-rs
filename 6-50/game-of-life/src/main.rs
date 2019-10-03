use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let r: usize = board.len();
        let c: usize = board[0].len();
        for i in 0..r {
            for j in 0..c {
                let mut one_count: i32 = 0;
                if i > 0 {
                    if board[i - 1][j] & 1 != 0 {
                        one_count += 1;
                    }
                    if j > 0 && board[i - 1][j - 1] & 1 != 0 {
                        one_count += 1;
                    }
                    if j + 1 < c && board[i - 1][j + 1] & 1 != 0 {
                        one_count += 1;
                    }
                }
                if i + 1 < r {
                    if board[i + 1][j] & 1 != 0 {
                        one_count += 1;
                    }
                    if j > 0 && board[i + 1][j - 1] & 1 != 0 {
                        one_count += 1;
                    }
                    if j + 1 < c && board[i + 1][j + 1] & 1 != 0 {
                        one_count += 1;
                    }
                }
                if j > 0 && board[i][j - 1] & 1 != 0 {
                    one_count += 1;
                }
                if j + 1 < c && board[i][j + 1] & 1 != 0 {
                    one_count += 1;
                }
                if board[i][j] == 0 {
                    if one_count == 3 {
                        board[i][j] = 4;
                    }
                } else {
                    if one_count < 2 || one_count > 3 {
                        board[i][j] = 3;
                    }
                }
            }
        }
        for i in 0..r {
            for j in 0..c {
                if board[i][j] == 3 {
                    board[i][j] = 0;
                } else if board[i][j] == 4 {
                    board[i][j] = 1;
                }
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rows = i32::from_str(&arg).expect("Error parse."),
            2 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if ret % columns == 0 {
                    board.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
        return;
    }

    Solution::game_of_life(&mut board);
    for v in board {
        for n in v {
            print!("{} ", n);
        }
        print!("\n");
    }
}
