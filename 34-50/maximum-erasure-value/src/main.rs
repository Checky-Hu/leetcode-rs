use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut set: HashSet<i32> = HashSet::new();
        let mut fast: usize = 0;
        let mut slow: usize = 0;
        let mut sum: i32 = 0;
        let mut result: i32 = 0;
        while fast < len {
            if set.contains(&nums[fast]) {
                set.remove(&nums[slow]);
                sum -= nums[slow];
                slow += 1;
            } else {
                set.insert(nums[fast]);
                sum += nums[fast];
                if sum > result {
                    result = sum;
                }
                fast += 1;
            }
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
        "Maximum unique subarray: {}",
        Solution::maximum_unique_subarray(nums)
    );
}
