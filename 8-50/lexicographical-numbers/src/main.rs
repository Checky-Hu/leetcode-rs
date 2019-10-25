use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n as usize);
        let mut num: i32 = 1;
        for _i in 0..n {
            result.push(num);
            if num * 10 <= n {
                num *= 10;
            } else {
                if num >= n {
                    num /= 10;
                }
                num += 1;
                while num % 10 == 0 {
                    num /= 10;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::lexical_order(n);
            for r in result {
                print!("{} ", r);
            }
            print!("\n");
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
