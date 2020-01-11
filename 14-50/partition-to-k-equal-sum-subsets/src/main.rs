extern crate quicksort;

use quicksort::qsi32;
use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_target(
        nums: &[i32],
        visited: &mut Vec<bool>,
        s: usize,
        cur: i32,
        target: i32,
        k: i32,
    ) -> bool {
        if k == 1 {
            return true;
        }
        match cur.cmp(&target) {
            Ordering::Greater => false,
            Ordering::Equal => Solution::get_target(nums, visited, 0, 0, target, k - 1),
            Ordering::Less => {
                for i in s..nums.len() {
                    if visited[i] {
                        continue;
                    }
                    visited[i] = true;
                    if Solution::get_target(nums, visited, s + 1, cur + nums[i], target, k) {
                        return true;
                    }
                    visited[i] = false;
                }
                false
            }
        }
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let mut sum: i32 = 0;
        for n in &nums {
            sum += *n;
        }
        if sum % k != 0 {
            false
        } else {
            let len: usize = nums.len();
            let mut tmp: Vec<i32> = nums;
            qsi32::quick_sort_descend(&mut tmp, 0, len - 1);
            let mut visited: Vec<bool> = vec![false; len];
            Solution::get_target(&tmp, &mut visited, 0, 0, sum / k, k)
        }
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Partition to k subsets: {}",
        Solution::can_partition_k_subsets(nums, k)
    );
}
