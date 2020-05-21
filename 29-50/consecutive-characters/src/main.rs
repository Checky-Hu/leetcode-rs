use std::env;

struct Solution {}

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut pre: (char, i32) = ('0', 1);
        let mut result: i32 = 0;
        for c in s.chars() {
            if c == pre.0 {
                pre.1 += 1;
            } else {
                if pre.1 > result {
                    result = pre.1;
                }
                pre.0 = c;
                pre.1 = 1;
            }
        }
        if pre.1 > result {
            result = pre.1;
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
            println!("Maximum power: {}", Solution::max_power(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
