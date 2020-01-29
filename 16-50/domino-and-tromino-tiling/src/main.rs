use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut dp: Vec<i32> = vec![0; n as usize + 2];
        dp[0] = 1;
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i as usize] = (2 * dp[i as usize - 1] % modulo + dp[i as usize - 3]) % modulo;
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
            println!("Ways to tile: {}", Solution::num_tilings(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
