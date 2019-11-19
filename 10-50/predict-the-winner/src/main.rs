use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let len: usize = nums.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len]; len];
        for i in 0..len {
            dp[i][i] = nums[i];
        }
        for i in 1..len {
            let mut j: usize = 0;
            let mut k: usize = i;
            while k < len {
                let t1: i32 = nums[j] - dp[j + 1][k];
                let t2: i32 = nums[k] - dp[j][k - 1];
                dp[j][k] = if t1 >= t2 {
                    t1
                } else {
                    t2
                };
                j += 1;
                k += 1;
            }
        }
        dp[0][len - 1] >= 0
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Player 1 win: {}", Solution::predict_the_winner(nums));
}
