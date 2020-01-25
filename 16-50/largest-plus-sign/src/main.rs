use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let len: usize = n as usize;
        let mut board: Vec<Vec<i32>> = vec![vec![1; len]; len];
        for mine in mines {
            board[mine[0] as usize][mine[1] as usize] = 0;
        }
        let mut result: i32 = 0;
        for i in 0..len {
            for j in 0..len {
                if board[i][j] == 0 {
                    continue;
                }
                let mut tmp: usize = 1;
                loop {
                    if i < tmp || board[i - tmp][j] == 0 {
                        break;
                    }
                    if i + tmp >= len || board[i + tmp][j] == 0 {
                        break;
                    }
                    if j < tmp || board[i][j - tmp] == 0 {
                        break;
                    }
                    if j + tmp >= len || board[i][j + tmp] == 0 {
                        break;
                    }
                    tmp += 1;
                }
                if tmp as i32 > result {
                    result = tmp as i32;
                }
            }
        }
        result
    }
}

fn main() {
    let mut n: i32 = 0;
    let mut mines: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(number);
                if tmp.len() == 2 {
                    mines.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == n {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Order of largest plus sign: {}",
        Solution::order_of_largest_plus_sign(n, mines)
    );
}
