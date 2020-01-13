use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; b_len + 1]; a_len + 1];
        for i in 1..=a_len {
            for j in 1..=b_len {
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    0
                };
                if result < dp[i][j] {
                    result = dp[i][j];
                }
            }
        }
        result
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
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() == n as usize {
                    b.push(t);
                } else {
                    a.push(t);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Max length of subarray: {}", Solution::find_length(a, b));
}
