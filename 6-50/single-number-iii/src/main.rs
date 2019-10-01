use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut diff: i32 = 0;
        for n in &nums {
            diff ^= *n;
        }
        diff &= 0 - diff;
        let mut result: Vec<i32> = vec![0; 2];
        for n in &nums {
            if *n & diff != 0 {
                result[0] ^= *n;
            } else {
                result[1] ^= *n;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::single_number(nums);
    println!("Number: {} {}", result[0], result[1]);
}
