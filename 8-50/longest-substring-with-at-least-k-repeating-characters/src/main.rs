use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn longest_substring_loop(b: &[u8], l: usize, r: usize, k: i32) -> i32 {
        if r + 1 - l < k as usize {
            return 0
        }
        let mut v: Vec<i32> = vec![0; 26];
        for i in l..=r {
            v[b[i] as usize - 97] += 1;
        }
        let mut is_valid: bool = true;
        let mut max_i: usize = l;
        let mut result: i32 = 0;
        for i in l..=r {
            if v[b[i] as usize - 97] < k {
                let t: i32 = if i == 0 {
                    0
                } else {
                    Solution::longest_substring_loop(b, max_i, i - 1, k)
                };
                if t > result {
                    result = t;
                }
                is_valid = false;
                max_i = i + 1;
            }
        }
        if is_valid {
            (r - l) as i32 + 1
        } else {
            let t: i32 = Solution::longest_substring_loop(b, max_i, r, k);
            if t > result {
                t
            } else {
                result
            }
        }
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let len: usize = s.len();
        if len == 0 {
            0
        } else {
            Solution::longest_substring_loop(s.as_bytes(), 0, len - 1, k)
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => {
                ret += 1;
                let s: String = arg;
                println!("Longest substring: {}", Solution::longest_substring(s, k));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
