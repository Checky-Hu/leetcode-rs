use std::env;

struct Solution {}

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut count_1: i32 = 0;
        for c in s.chars() {
            if c == '1' {
                count_1 += 1;
            }
        }
        let mut count_0: i32 = 0;
        let len: usize = s.len();
        let mut result: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            if i == len - 1 {
                break;
            }
            if c == '0' {
                count_0 += 1;
            } else {
                count_1 -= 1;
            }
            let t: i32 = count_0 + count_1;
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Maximum score: {}", Solution::max_score(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
