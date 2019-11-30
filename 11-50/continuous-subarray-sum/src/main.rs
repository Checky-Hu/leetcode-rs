use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        let mut pre: i32 = 0;
        let mut sum: i32 = 0;
        for n in nums {
            sum += n;
            let t: i32 = if k == 0 {
                sum
            } else {
                sum % k
            };
            if set.contains(&t) {
                return true
            } else {
                set.insert(pre);
                pre = t;
            }
        }
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    println!("Subarray sum contains {}: {}", k, Solution::check_subarray_sum(nums, k));
}
