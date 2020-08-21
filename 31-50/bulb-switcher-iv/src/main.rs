use std::env;

struct Solution {}

impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut prefix_c: char = '0';
        let mut result: i32 = 0;
        for c in target.chars() {
            if c != prefix_c {
                result += 1;
                prefix_c = c;
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
            let target: String = arg;
            println!("Minimum flips: {}", Solution::min_flips(target));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
