use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut result: String = String::new();
        let t: i32 = if n & 1 == 1 {
            n
        } else {
            result.push('z');
            n - 1
        };
        for _i in 0..t {
            result.push('a');
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Generate string: {}", Solution::generate_the_string(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
