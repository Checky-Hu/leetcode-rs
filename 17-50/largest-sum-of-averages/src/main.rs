use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        let len: usize = a.len();
        let mut sum: Vec<i32> = vec![0i32; len + 1];
        for i in 0..len {
            sum[i + 1] = sum[i] + a[i];
        }
        let mut dp: Vec<Vec<f64>> = vec![vec![0f64; k as usize]; len];
        for i in 0..len {
            dp[i][0] = (sum[len] - sum[i]) as f64 / (len - i) as f64;
        }
        for parts in 1..k {
            for i in 0..(len - 1) {
                for j in (i + 1)..len {
                    let tmp: f64 =
                        (sum[j] - sum[i]) as f64 / (j - i) as f64 + dp[j][parts as usize - 1];
                    if tmp > dp[i][parts as usize] {
                        dp[i][parts as usize] = tmp;
                    }
                }
            }
        }
        dp[0][k as usize - 1]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(t);
            }
        }
    }

    if 0 == k || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Largest sum of averages: {}",
        Solution::largest_sum_of_averages(a, k)
    );
}
