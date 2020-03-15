use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; b_len + 1]; a_len + 1];
        for i in 0..a_len {
            for j in 0..b_len {
                dp[i + 1][j + 1] = if a[i] == b[j] {
                    dp[i][j] + 1
                } else if dp[i + 1][j] > dp[i][j + 1] {
                    dp[i + 1][j]
                } else {
                    dp[i][j + 1]
                };
            }
        }
        dp[a_len][b_len]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() == n as usize {
                    b.push(number);
                } else {
                    a.push(number);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Maximum uncrossed lines: {}",
        Solution::max_uncrossed_lines(a, b)
    );
}
