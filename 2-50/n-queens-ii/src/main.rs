use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn total_n_queens_loop(queens: &mut Vec<(i32, i32)>, i: i32, n: i32) -> i32 {
        if i == n {
            1
        } else {
            let mut result: i32 = 0;
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
                    result += Solution::total_n_queens_loop(queens, i + 1, n);
                    queens.pop();
                }
            }
            result
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut queens: Vec<(i32, i32)> = Vec::with_capacity(n as usize);
        Solution::total_n_queens_loop(&mut queens, 0, n)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "{} queens total solutions: {}",
                n,
                Solution::total_n_queens(n)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
