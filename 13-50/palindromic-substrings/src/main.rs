use std::env;

struct Solution {}

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut result: i32 = 0;
        let mut i: usize = len - 1;
        loop {
            for j in i..len {
                dp[i][j] = (bytes[i] == bytes[j]) && (j - i <= 2 || dp[i + 1][j - 1]);
                if dp[i][j] {
                    result += 1;
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Palindromic substrings count: {}",
                Solution::count_substrings(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
