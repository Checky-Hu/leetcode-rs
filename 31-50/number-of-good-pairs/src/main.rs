use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counts: Vec<i32> = vec![0; 100];
        for num in nums.iter() {
            counts[*num as usize - 1] += 1;
        }
        let mut result: i32 = 0;
        for count in counts.iter() {
            if *count > 1 {
                result += *count * (*count - 1) / 2;
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
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Number of good pairs: {}",
        Solution::num_identical_pairs(nums)
    );
}
