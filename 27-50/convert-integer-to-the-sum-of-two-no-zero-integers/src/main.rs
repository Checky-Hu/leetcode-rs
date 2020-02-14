use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn contains_zero(n: i32) -> bool {
        let mut t: i32 = n;
        while t > 0 {
            if t % 10 == 0 {
                return true;
            } else {
                t /= 10;
            }
        }
        false
    }

    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for i in 1..=(n / 2) {
            if !Solution::contains_zero(i) && !Solution::contains_zero(n - i) {
                result.push(i);
                result.push(n - i);
                break;
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
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::get_no_zero_integers(n);
            for r in &result {
                print!("{} ", *r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
