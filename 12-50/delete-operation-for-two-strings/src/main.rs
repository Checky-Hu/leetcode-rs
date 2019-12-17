use std::env;

struct Solution {
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let b1: &[u8] = word1.as_bytes();
        let len1: usize = b1.len();
        let b2: &[u8] = word2.as_bytes();
        let len2: usize = b2.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 1..=len1 {
            for j in 1..=len2 {
                dp[i][j] = if b1[i - 1] == b2[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    if dp[i - 1][j] > dp[i][j - 1] {
                        dp[i - 1][j]
                    } else {
                        dp[i][j - 1]
                    }
                };
            }
        }
        (len1 + len2) as i32 - 2 * dp[len1][len2]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut word1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            word1 = arg;
        } else if 2 == index {
            ret += 1;
            let word2: String = arg;
            println!("Min distance: {}", Solution::min_distance(word1, word2));
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
