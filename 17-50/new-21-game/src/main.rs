use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn new21_game(n: i32, k: i32, w: i32) -> f64 {
        if k == 0 || n >= k + w {
            return 1f64;
        }
        let mut sum: Vec<f64> = vec![0f64; (k + w) as usize];
        sum[0] = 1f64;
        for i in 1..(k + w) {
            let t: usize = if i < k { i as usize } else { k as usize } - 1;
            if i <= w {
                sum[i as usize] = sum[i as usize - 1] + sum[t] / (w as f64);
            } else {
                sum[i as usize] =
                    sum[i as usize - 1] + (sum[t] - sum[(i - w) as usize - 1]) / (w as f64);
            }
        }
        (sum[n as usize] - sum[k as usize - 1]) / (sum[(k + w) as usize - 1] - sum[k as usize - 1])
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let w: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Probability: {}", Solution::new21_game(n, k, w));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
