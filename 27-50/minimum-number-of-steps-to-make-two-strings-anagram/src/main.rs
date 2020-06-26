use std::env;

struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count_s: Vec<i32> = vec![0; 26];
        for u in s.as_bytes() {
            count_s[*u as usize - 97] += 1;
        }
        let mut count_t: Vec<i32> = vec![0; 26];
        for u in t.as_bytes() {
            count_t[*u as usize - 97] += 1;
        }
        let mut result: i32 = 0;
        for i in 0..26 {
            let diff: i32 = count_s[i] - count_t[i];
            if diff > 0 {
                result += diff;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: String = arg;
                println!("Minimum steps for anagram: {}", Solution::min_steps(s, t));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
