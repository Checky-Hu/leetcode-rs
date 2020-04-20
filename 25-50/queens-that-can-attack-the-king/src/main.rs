use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut board: Vec<Vec<bool>> = vec![vec![false; 8]; 8];
        for queen in queens.iter() {
            board[queen[0] as usize][queen[1] as usize] = true;
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        let directions: Vec<(i32, i32)> = vec![
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
        ];
        for direction in directions.iter() {
            let mut position: (i32, i32) = (king[0], king[1]);
            loop {
                position.0 += direction.0;
                position.1 += direction.1;
                if position.0 < 0 || position.0 >= 8 || position.1 < 0 || position.1 >= 8 {
                    break;
                } else if board[position.0 as usize][position.1 as usize] {
                    result.push(vec![position.0, position.1]);
                    break;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut king: Vec<i32> = Vec::new();
    let mut queens: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if king.len() == 2 {
                    tmp.push(n);
                    if tmp.len() == 2 {
                        queens.push(tmp);
                        tmp = Vec::new();
                    }
                } else {
                    king.push(n);
                }
            }
        }
    }

    if 0 == ret || 0 != ret & 1 {
        println!("Require at least (2 * n) parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::queens_attackthe_king(queens, king);
    for r in result.iter() {
        print!("[{}, {}] ", r[0], r[1]);
    }
    println!();
}
