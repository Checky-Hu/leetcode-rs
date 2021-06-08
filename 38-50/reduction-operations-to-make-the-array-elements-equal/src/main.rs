use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort_by(|a, b| b.cmp(&a));
        let mut result: i32 = 0;
        // (value, count)
        let mut prefix: (i32, i32) = (0, 0);
        for v in nums_mut.iter() {
            if prefix.0 != *v {
                result += prefix.1;
                prefix.0 = *v;
            }
            prefix.1 += 1;
        }
        result
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
        "Reduction operations: {}",
        Solution::reduction_operations(nums)
    );
}
