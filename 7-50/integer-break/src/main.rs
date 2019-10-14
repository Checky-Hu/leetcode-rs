use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 || n == 3 {
            return n - 1
        }
        let mut result: i32 = 1;
        let mut tmp: i32 = n;
        while tmp > 4 {
            result *= 3;
            tmp -= 3;
        }
        result * tmp
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Max product after break: {}", Solution::integer_break(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
