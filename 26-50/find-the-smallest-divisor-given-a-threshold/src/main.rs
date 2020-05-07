use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_threshold(nums: &[i32], k: i32) -> i32 {
        let mut result: i32 = 0;
        for n in nums.iter() {
            result += (*n + k - 1) / k;
        }
        result
    }

    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut min: i32 = i32::max_value();
        let mut max: i32 = i32::min_value();
        for n in nums.iter() {
            if *n > max {
                max = *n;
            }
            if *n < min {
                min = *n;
            }
        }
        let mut left: i32 = min * nums.len() as i32 / threshold;
        if left == 0 {
            left = 1;
        }
        let mut right: i32 = max;
        while left <= right {
            let mid: i32 = left + (right - left) / 2;
            let tmp: i32 = Solution::count_threshold(&nums, mid);
            if tmp > threshold {
                left = mid + 1;
            } else if mid == 1 || Solution::count_threshold(&nums, mid - 1) > threshold {
                return mid as i32;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut threshold: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => threshold = i32::from_str(&arg).expect("Error parse."),
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
        "Smallest divisor: {}",
        Solution::smallest_divisor(nums, threshold)
    );
}
