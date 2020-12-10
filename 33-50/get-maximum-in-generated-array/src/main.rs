use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums: Vec<i32> = Vec::with_capacity(101);
        let mut result: i32 = 0;
        for i in 0..=n {
            let t: i32 = match i {
                0 => 0,
                1 => 1,
                _ => {
                    if i & 1 == 0 {
                        nums[(i >> 1) as usize]
                    } else {
                        let j: usize = ((i - 1) >> 1) as usize;
                        nums[j] + nums[j + 1]
                    }
                }
            };
            nums.push(t);
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Maximum generated: {}", Solution::get_maximum_generated(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
