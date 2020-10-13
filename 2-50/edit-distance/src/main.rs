use std::env;

struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let bytes1: &[u8] = word1.as_bytes();
        let len1: usize = bytes1.len();
        let bytes2: &[u8] = word2.as_bytes();
        let len2: usize = bytes2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len1 + 1]; len2 + 1];
        for (i, v) in dp[0].iter_mut().enumerate() {
            *v = i as i32;
        }
        for (i, r) in dp.iter_mut().enumerate() {
            r[0] = i as i32;
        }
        for i in 1..=len2 {
            for j in 1..=len1 {
                if bytes1[j - 1] == bytes2[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + std::cmp::min(
                        dp[i - 1][j - 1],
                        std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                    );
                }
            }
        }
        dp[len2][len1]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut word1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => word1 = arg,
            _ => {
                ret += 1;
                let word2: String = arg;
                println!("Min distance: {}", Solution::min_distance(word1, word2));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
