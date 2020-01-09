use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        if k == 0 {
            return 1.0_f64;
        }
        let mut dp: Vec<Vec<f64>> = vec![vec![1.0; n as usize]; n as usize];
        let directions: Vec<Vec<i32>> = vec![
            vec![-2, -1],
            vec![-1, -2],
            vec![-2, 1],
            vec![1, -2],
            vec![-1, 2],
            vec![2, -1],
            vec![1, 2],
            vec![2, 1],
        ];
        for _t in 0..k {
            let mut cur: Vec<Vec<f64>> = vec![vec![0.0; n as usize]; n as usize];
            for i in 0..n {
                for j in 0..n {
                    for direction in &directions {
                        let pos_x: i32 = i + direction[0];
                        let pos_y: i32 = j + direction[1];
                        if pos_x < 0 || pos_x >= n || pos_y < 0 || pos_y >= n {
                            continue;
                        } else {
                            cur[i as usize][j as usize] += dp[pos_x as usize][pos_y as usize];
                        }
                    }
                }
            }
            dp = cur;
        }
        dp[r as usize][c as usize] as f64 / 8.0_f64.powf(k as f64)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut r: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            3 => r = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Knight probability: {}",
                    Solution::knight_probability(n, k, r, c)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least four parameters.");
    }
}
