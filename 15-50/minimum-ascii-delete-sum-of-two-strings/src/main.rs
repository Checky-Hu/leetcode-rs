use std::env;

struct Solution {}

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1_bytes: &[u8] = s1.as_bytes();
        let len1: usize = s1_bytes.len();
        let s2_bytes: &[u8] = s2.as_bytes();
        let len2: usize = s2_bytes.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 1..=len2 {
            dp[0][i] = dp[0][i - 1] + s2_bytes[i - 1] as i32;
        }
        for i in 1..=len1 {
            dp[i][0] = dp[i - 1][0] + s1_bytes[i - 1] as i32;
            for j in 1..=len2 {
                dp[i][j] = if s1_bytes[i - 1] == s2_bytes[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    let t1: i32 = dp[i - 1][j] + s1_bytes[i - 1] as i32;
                    let t2: i32 = dp[i][j - 1] + s2_bytes[j - 1] as i32;
                    if t1 > t2 {
                        t2
                    } else {
                        t1
                    }
                };
            }
        }
        dp[len1][len2]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s1: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s1 = arg,
            _ => {
                ret += 1;
                let s2: String = arg;
                println!(
                    "Minimum delete sum: {}",
                    Solution::minimum_delete_sum(s1, s2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
