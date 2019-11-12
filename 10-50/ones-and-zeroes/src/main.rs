use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize + 1]; m as usize + 1];
        for s in strs {
            let mut z_count: i32 = 0;
            let mut o_count: i32 = 0;
            for c in s.chars() {
                if c == '0' {
                    z_count += 1;
                } else {
                    o_count += 1;
                }
            }
            let mut i: i32 = m;
            while i >= z_count {
                let mut j: i32 = n;
                while j >= o_count {
                    if dp[i as usize][j as usize] < dp[(i - z_count) as usize][(j - o_count) as usize] + 1 {
                        dp[i as usize][j as usize] = dp[(i - z_count) as usize][(j - o_count) as usize] + 1;
                    }
                    j -= 1;
                }
                i -= 1;
            }
        }
        dp[m as usize][n as usize]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut strs: Vec<String> = Vec::new();
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                strs.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Max form: {}",  Solution::find_max_form(strs, m, n));
}
