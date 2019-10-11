use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut fri: i32 = i32::max_value();
        let mut sec: i32 = i32::max_value();
        for n in nums {
            if n <= fri {
                fri = n;
            } else if n <= sec {
                sec = n;
            } else {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Found increasing triplet: {}", Solution::increasing_triplet(nums));
}
