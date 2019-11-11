extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    fn make_square_loop(nums: &Vec<i32>, len: usize, pos: usize, result: &mut Vec<i32>, target: i32) -> bool {
        if pos == len {
            return result[0] == target && result[1] == target && result[2] == target
        }
        for i in 0..4 {
            if nums[pos] + result[i] > target {
                continue;
            }
            result[i] += nums[pos];
            if Solution::make_square_loop(nums, len, pos + 1, result, target) {
                return true
            }
            result[i] -= nums[pos];
        }
        false
    }

    pub fn makesquare(nums: Vec<i32>) -> bool {
        let mut sum: i32 = 0;
        for num in &nums {
            sum += num;
        }
        if sum % 4 != 0 {
            return false
        }
        let target: i32 = sum / 4;
        let mut result: Vec<i32> = vec![0; 4];
        let len: usize = nums.len();
        if len == 0 {
            return false
        }
        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort_descend(&mut tmp, 0, len - 1);
        Solution::make_square_loop(&tmp, len, 0, &mut result, target)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Make square: {}", Solution::makesquare(nums));
}
