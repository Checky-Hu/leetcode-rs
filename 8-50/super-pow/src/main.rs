use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn pow(x: i32, n: i32) -> i32 {
        match n {
            0 => 1,
            1 => x % 1337,
            _ => Solution::pow(x, n / 2) * Solution::pow(x, n - n / 2) % 1337,
        }
    }

    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut result: i32 = 1;
        for i in b {
            result = Solution::pow(result, 10) * Solution::pow(a, i) % 1337;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: i32 = 0;
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                b.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return
    }

    println!("Result: {}", Solution::super_pow(a, b));
}
