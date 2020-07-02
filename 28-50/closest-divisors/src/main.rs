use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_divisors(num: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![1, num];
        let sqrt: i32 = (num as f64).sqrt().trunc() as i32;
        for i in 1..=sqrt {
            if num % i == 0 {
                result[0] = i;
                result[1] = num / i;
            }
        }
        result
    }

    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let r1: Vec<i32> = Solution::get_divisors(num + 1);
        let r2: Vec<i32> = Solution::get_divisors(num + 2);
        if r1[1] - r1[0] > r2[1] - r2[0] {
            r2
        } else {
            r1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::closest_divisors(num);
            println!("Closest divisors: [{}, {}]", result[0], result[1]);
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
