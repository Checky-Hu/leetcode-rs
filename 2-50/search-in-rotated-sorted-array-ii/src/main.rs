use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn search_loop(nums: &[i32], s: usize, e: usize, target: i32) -> bool {
        if s == e {
            nums[s] == target
        } else if s + 1 == e {
            nums[s] == target || nums[e] == target
        } else {
            let index: usize = s + (e - s) / 2;
            if nums[index] == target {
                true
            } else {
                let left_sorted: bool = nums[s] <= nums[index];
                let right_sorted: bool = nums[index] <= nums[e];
                if left_sorted {
                    if right_sorted {
                        Solution::search_loop(nums, s, index - 1, target)
                            || Solution::search_loop(nums, index + 1, e, target)
                    } else if nums[s] <= target && target < nums[index] {
                        Solution::search_loop(nums, s, index - 1, target)
                    } else {
                        Solution::search_loop(nums, index + 1, e, target)
                    }
                } else if nums[index] < target && target <= nums[e] {
                    Solution::search_loop(nums, index + 1, e, target)
                } else {
                    Solution::search_loop(nums, s, index - 1, target)
                }
            }
        }
    }

    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let len: usize = nums.len();
        if len == 0 {
            false
        } else {
            Solution::search_loop(&nums, 0, len - 1, target)
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Has target {}: {}", target, Solution::search(nums, target));
}
