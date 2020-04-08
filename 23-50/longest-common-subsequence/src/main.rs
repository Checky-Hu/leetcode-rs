use std::env;

struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let bytes1: &[u8] = text1.as_bytes();
        let len1: usize = bytes1.len();
        let bytes2: &[u8] = text2.as_bytes();
        let len2: usize = bytes2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 1..=len1 {
            for j in 1..=len2 {
                dp[i][j] = if bytes1[i - 1] == bytes2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else if dp[i - 1][j] > dp[i][j - 1] {
                    dp[i - 1][j]
                } else {
                    dp[i][j - 1]
                };
            }
        }
        dp[len1][len2]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut text1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => text1 = arg,
            2 => {
                ret += 1;
                let text2: String = arg;
                println!(
                    "Longest common subsequence: {}",
                    Solution::longest_common_subsequence(text1, text2)
                );
                break;
            }
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
