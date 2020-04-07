use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let len: usize = piles.len();
        let mut sum: Vec<i32> = vec![0; len + 1];
        let mut i: usize = len - 1;
        loop {
            sum[i] = sum[i + 1] + piles[i];
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len + 1]; len + 1];
        i = len - 1;
        loop {
            for j in 1..=len {
                for k in 1..=(2 * j) {
                    if i + k > len {
                        break;
                    }
                    let t: i32 = sum[i] - dp[i + k][if j > k { j } else { k }];
                    if t > dp[i][j] {
                        dp[i][j] = t;
                    }
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        dp[0][1]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut piles: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                piles.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Maximum number of stones that Alex can get: {}",
        Solution::stone_game_ii(piles)
    );
}
