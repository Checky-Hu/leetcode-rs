use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        if n == 1 {
            0
        } else if n & 1 == 0 {
            1 + Solution::integer_replacement(n / 2)
        } else {
            let t: i64 = n as i64;
            let r1: i32 = Solution::integer_replacement(((t + 1) / 2) as i32);
            let r2: i32 = Solution::integer_replacement(((t - 1) / 2) as i32);
            if r1 >= r2 {
                r2 + 2
            } else {
                r1 + 2
            }
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Replacement: {}", Solution::integer_replacement(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
