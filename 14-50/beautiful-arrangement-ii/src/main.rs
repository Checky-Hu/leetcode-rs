use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(n as usize);
        result.push(n);
        let mut t: i32 = n;
        let mut i: i32 = k;
        let mut is_add: bool = false;
        while i > 0 {
            if is_add {
                t += i;
            } else {
                t -= i;
            }
            result.push(t);
            is_add = !is_add;
            i -= 1;
        }
        t = n - k - 1;
        while t > 0 {
            result.push(t);
            t -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::construct_array(n, k);
                for r in &result {
                    print!("{} ", *r);
                }
                println!();
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
