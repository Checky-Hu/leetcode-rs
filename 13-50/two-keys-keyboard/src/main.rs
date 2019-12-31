use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; n as usize + 1];
        for i in 2..=n {
            dp[i as usize] = i;
            let mut j: i32 = i - 1;
            while j > 1 {
                if i % j == 0 {
                    let t: i32 = dp[j as usize] + i / j;
                    if t < dp[i as usize] {
                        dp[i as usize] = t;
                    }
                    break;
                } else {
                    j -= 1;
                }
            }
        }
        dp[n as usize]
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Min steps: {}", Solution::min_steps(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
