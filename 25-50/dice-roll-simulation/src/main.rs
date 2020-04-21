use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 7]; n as usize + 1];
        for i in 1..=n {
            for j in 0..6 {
                if i == 1 && roll_max[j] > 0 {
                    dp[i as usize][j] = 1;
                } else if i <= roll_max[j] {
                    dp[i as usize][j] = dp[i as usize - 1][6];
                } else if i == roll_max[j] + 1 {
                    dp[i as usize][j] = dp[i as usize - 1][6] - 1;
                } else {
                    dp[i as usize][j] = (dp[i as usize - 1][6]
                        - dp[(i - roll_max[j]) as usize - 1][6]
                        + dp[(i - roll_max[j]) as usize - 1][j])
                        % modulo;
                    if dp[i as usize][j] < 0 {
                        dp[i as usize][j] += modulo;
                    }
                }
                dp[i as usize][6] = (dp[i as usize][6] + dp[i as usize][j]) % modulo;
            }
        }
        dp[n as usize][6]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut roll_max: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                roll_max.push(number);
            }
        }
    }

    if 6 != ret {
        println!("Require at least seven parameters.");
        return;
    }

    println!("Die simulator: {}", Solution::die_simulator(n, roll_max));
}
