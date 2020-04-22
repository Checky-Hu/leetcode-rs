use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn nth_person_gets_nth_seat(n: i32) -> f64 {
        let mut dp: Vec<f64> = Vec::with_capacity(n as usize);
        dp.push(1f64);
        let mut sum: f64 = 0f64;
        for i in 1..n {
            dp.push((1f64 + sum) / (i + 1) as f64);
            sum += dp[i as usize];
        }
        dp[n as usize - 1]
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Probability: {}", Solution::nth_person_gets_nth_seat(n));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
