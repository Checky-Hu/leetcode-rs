use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_paths(m: i32, n: i32, N: i32, i: i32, j: i32) -> i32 {
        let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; n as usize]; m as usize]; 1 + N as usize];
        for k in 1..=N {
            for x in 0..m {
                for y in 0..n {
                    let up: i64 = if x == 0 {
                        1
                    } else {
                        dp[k as usize - 1][x as usize - 1][y as usize]
                    };
                    let down: i64 = if x == m - 1 {
                        1
                    } else {
                        dp[k as usize - 1][x as usize + 1][y as usize]
                    };
                    let left: i64 = if y == 0 {
                        1
                    } else {
                        dp[k as usize - 1][x as usize][y as usize - 1]
                    };
                    let right: i64 = if y == n - 1 {
                        1
                    } else {
                        dp[k as usize - 1][x as usize][y as usize + 1]
                    };
                    dp[k as usize][x as usize][y as usize] = (up + down + left + right) % 1000000007;
                }
            }
        }
        dp[N as usize][i as usize][j as usize] as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut N: i32 = 0;
    let mut i: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
                ret += 1;
                m = i32::from_str(&arg).expect("Error parse.");
            },
            2 => {
                ret += 1;
                n = i32::from_str(&arg).expect("Error parse.");
            },
            3 => {
                ret += 1;
                N = i32::from_str(&arg).expect("Error parse.");
            },
            4 => {
                ret += 1;
                i = i32::from_str(&arg).expect("Error parse.");
            },
            5 => {
                ret += 1;
                let j: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Out of boundary paths: {}", Solution::find_paths(m, n, N, i, j));
                break;
            },
            _ => (),
        }
    }

    if 5 > ret {
        println!("Require five parameters.");
    }
}
