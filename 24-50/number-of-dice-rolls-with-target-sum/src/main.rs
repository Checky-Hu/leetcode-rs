use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; target as usize + 1]; d as usize + 1];
        for i in 1..=target {
            if i > f {
                break;
            }
            dp[1][i as usize] = 1;
        }
        for i in 2..=d {
            for j in 1..=target {
                for k in 1..=f {
                    if k > j {
                        break;
                    }
                    dp[i as usize][j as usize] = (dp[i as usize][j as usize]
                        + dp[i as usize - 1][(j - k) as usize])
                        % modulo;
                }
            }
        }
        dp[d as usize][target as usize]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut d: i32 = 0;
    let mut f: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => d = i32::from_str(&arg).expect("Error parse."),
            2 => f = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let target: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Number of rolls to target: {}",
                    Solution::num_rolls_to_target(d, f, target)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
    }
}
