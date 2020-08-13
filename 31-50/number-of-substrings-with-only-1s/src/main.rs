use std::env;

struct Solution {}

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let mut count: i64 = 0;
        let mut result: i64 = 0;
        for c in s.chars() {
            if c == '1' {
                count += 1;
            } else if count > 0 {
                result = (result + (count + 1) * count / 2) % modulo;
                count = 0;
            }
        }
        if count > 0 {
            result = (result + (count + 1) * count / 2) % modulo;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Number of substrings: {}", Solution::num_sub(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
