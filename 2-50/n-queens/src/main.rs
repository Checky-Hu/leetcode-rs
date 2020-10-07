use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn solve_n_queens_loop(
        queens: &mut Vec<(i32, i32)>,
        i: i32,
        n: i32,
        result: &mut Vec<Vec<String>>,
    ) {
        if i == n {
            let mut board: Vec<String> = Vec::with_capacity(n as usize);
            for queen in queens.iter() {
                let mut row: Vec<u8> = vec![b'.'; n as usize];
                row[queen.1 as usize] = b'Q';
                board.push(String::from_utf8(row).unwrap_or_default());
            }
            result.push(board);
        } else {
            for j in 0..n {
                let mut is_valid: bool = true;
                for queen in queens.iter() {
                    if j == queen.1 {
                        is_valid = false;
                        break;
                    }
                    if i - queen.0 == (j - queen.1).abs() {
                        is_valid = false;
                        break;
                    }
                }
                if is_valid {
                    queens.push((i, j));
                    Solution::solve_n_queens_loop(queens, i + 1, n, result);
                    queens.pop();
                }
            }
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut queens: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
        let mut result: Vec<Vec<String>> = Vec::new();
        for i in 0..n {
            queens.push((0, i));
            Solution::solve_n_queens_loop(&mut queens, 1, n, &mut result);
            queens.pop();
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result = Solution::solve_n_queens(n);
            for row in result.iter() {
                for v in row.iter() {
                    println!("{}", *v);
                }
                println!();
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
