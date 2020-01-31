use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp: Vec<Vec<f64>> = vec![vec![0f64; 101]; 101];
        dp[0][0] = poured as f64;
        for i in 0..=query_row {
            for j in 0..=i {
                if dp[i as usize][j as usize] > 1f64 {
                    let tmp: f64 = (dp[i as usize][j as usize] - 1f64) / 2f64;
                    dp[i as usize + 1][j as usize] += tmp;
                    dp[i as usize + 1][j as usize + 1] += tmp;
                }
            }
        }
        if dp[query_row as usize][query_glass as usize] > 1f64 {
            1f64
        } else {
            dp[query_row as usize][query_glass as usize]
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut poured: i32 = 0;
    let mut query_row: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => poured = i32::from_str(&arg).expect("Error parse."),
            2 => query_row = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let query_glass: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "[{}, {}] after poured {}: {}",
                    query_row,
                    query_glass,
                    poured,
                    Solution::champagne_tower(poured, query_row, query_glass)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
    }
}
