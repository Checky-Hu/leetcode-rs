use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; 20_001];
        let mut result: i32 = 0;
        for a in arr.iter() {
            let current: i32 = *a + 10_000;
            let prefix: i32 = current - difference;
            if prefix < 0 || prefix >= 20_001 {
                dp[current as usize] = 1;
            } else {
                dp[current as usize] = dp[prefix as usize] + 1;
            }
            if dp[current as usize] > result {
                result = dp[current as usize];
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut difference: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => difference = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Longest subsequence: {}",
        Solution::longest_subsequence(arr, difference)
    );
}
