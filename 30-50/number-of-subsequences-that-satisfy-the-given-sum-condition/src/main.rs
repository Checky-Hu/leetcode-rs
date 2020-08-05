use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut mut_nums: Vec<i32> = nums;
        mut_nums.sort();
        let len: usize = mut_nums.len();
        let mut pow: Vec<i32> = vec![1; len];
        for i in 1..len {
            pow[i] = pow[i - 1] << 1;
            if pow[i] > modulo {
                pow[i] %= modulo;
            }
        }
        let mut left: usize = 0;
        let mut right: usize = len - 1;
        let mut result: i32 = 0;
        while left <= right {
            if mut_nums[left] + mut_nums[right] <= target {
                result = (result + pow[right - left]) % modulo;
                left += 1;
            } else if right == 0 {
                break;
            } else {
                right -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Number of subsequences: {}",
        Solution::num_subseq(nums, target)
    );
}
