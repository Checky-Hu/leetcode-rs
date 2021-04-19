use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for v in nums.iter() {
            match v.cmp(&0) {
                Ordering::Less => count += 1,
                Ordering::Equal => return 0,
                Ordering::Greater => (),
            }
        }
        if count & 1 == 1 {
            -1
        } else {
            1
        }
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
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Sign of the product of an array: {}",
        Solution::array_sign(nums)
    );
}
