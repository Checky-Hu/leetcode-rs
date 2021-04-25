use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut result: i32 = 0;
        let mut modulo: i32 = k;
        while modulo * k <= n {
            modulo *= k;
        }
        let mut number: i32 = n;
        while number > 0 {
            let t: i32 = number / modulo;
            number -= t * modulo;
            modulo /= k;
            result += t;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Sum of digits in base k: {}", Solution::sum_base(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
