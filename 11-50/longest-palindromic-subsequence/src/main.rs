use std::env;

struct Solution {
}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        if len == 0 {
            return 0
        }
        let mut dp: Vec<i32> = vec![1; len];
        let mut i: usize = len - 1;
        loop {
            let mut cur: i32 = 0;
            for j in (i + 1)..len {
                let tmp: i32 = dp[j];
                if bytes[i] == bytes[j] {
                    dp[j] = cur + 2;
                }
                if cur < tmp {
                    cur = tmp;
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let mut result: i32 = 0;
        for n in dp {
            if n > result {
                result = n;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Longest palindrome subseq: {}", Solution::longest_palindrome_subseq(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
