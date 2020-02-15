use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let len: usize = piles.len();
        let mut sum: i32 = 0;
        for v in piles.iter() {
            sum += *v;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len + 1]; len + 1];
        let mut i: usize = len - 2;
        loop {
            dp[i][i + 1] = if piles[i] > piles[i + 1] {
                piles[i]
            } else {
                piles[i + 1]
            };
            let mut j: usize = i + 3;
            while j < len {
                let t1: i32 = dp[i][j - 2]
                    + if piles[j - 1] > piles[j] {
                        piles[j - 1]
                    } else {
                        piles[j]
                    };
                let t2: i32 = dp[i + 2][j]
                    + if piles[i] > piles[i + 1] {
                        piles[i]
                    } else {
                        piles[i + 1]
                    };
                let t3: i32 = dp[i + 1][j - 1]
                    + if piles[i] > piles[j] {
                        piles[i]
                    } else {
                        piles[j]
                    };
                dp[i][j] = if t1 > t2 {
                    if t1 > t3 {
                        t1
                    } else {
                        t3
                    }
                } else {
                    if t2 > t3 {
                        t2
                    } else {
                        t3
                    }
                };
                j += 2;
            }
            if i == 0 {
                break;
            } else {
                i -= 2;
            }
        }
        dp[0][len - 1] > sum - dp[0][len - 1]
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
        "First player win stone game: {}",
        Solution::stone_game(piles)
    );
}
