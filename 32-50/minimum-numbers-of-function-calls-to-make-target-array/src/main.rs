use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut mut_nums: Vec<i32> = nums;
        let mut should_divide: bool = false;
        let mut result: i32 = 0;
        loop {
            let mut is_done: bool = true;
            for num in mut_nums.iter_mut() {
                if should_divide {
                    *num >>= 1;
                }
                if *num & 1 == 1 {
                    *num -= 1;
                    result += 1;
                }
                if *num != 0 {
                    is_done = false;
                }
            }
            if is_done {
                break;
            } else {
                should_divide = true;
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Minimum operations: {}", Solution::min_operations(nums));
}
