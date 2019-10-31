use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        if board.is_empty() || board[0].is_empty() {
            return 0
        }
        let mut result: i32 = 0;
        let r: usize = board.len();
        let c: usize = board[0].len();
        for i in 0..r {
            for j in 0..c {
                if board[i][j] == '.' || (i > 0 && board[i - 1][j] == 'X')
                    || (j > 0 && board[i][j - 1] == 'X') {
                    continue;
                } else {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rows = i32::from_str(&arg).expect("Error parse."),
            2 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let bytes: &[u8] = arg.as_bytes();
                tmp_row.push(bytes[0] as char);
                if tmp_row.len() == columns as usize {
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

    println!("Count: {}", Solution::count_battleships(board));
}
