use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort_unstable();
        nums_mut[len - 1] * nums_mut[len - 2] - nums_mut[0] * nums_mut[1]
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

    if 4 > ret {
        println!("Require at least 4 parameters.");
        return;
    }

    println!(
        "Maximum product difference: {}",
        Solution::max_product_difference(nums)
    );
}
