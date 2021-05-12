use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let len: usize = nums1.len();
        let mut sum: i64 = 0;
        for i in 0..len {
            sum += (nums1[i] - nums2[i]).abs() as i64;
        }
        if sum == 0 {
            return 0;
        }
        let mut mut_nums: Vec<i32> = nums1.clone();
        mut_nums.sort_unstable();
        let mut result: i64 = sum;
        for (i, n) in nums2.iter().enumerate() {
            let mut left: usize = 0;
            let mut right: usize = len - 1;
            while left < right {
                let middle: usize = (right - left) / 2 + left;
                if mut_nums[middle] <= *n {
                    if middle + 1 < len && mut_nums[middle + 1] > *n {
                        left = middle;
                        break;
                    } else {
                        left = middle + 1;
                    }
                } else if middle == 0 {
                    left = 0;
                    break;
                } else {
                    right = middle - 1;
                }
            }
            let diff1: i64 = (mut_nums[left] - nums2[i]).abs() as i64;
            let diff2: i64 = if left + 1 < len {
                (mut_nums[left + 1] - nums2[i]).abs() as i64
            } else {
                diff1
            };
            let tmp: i64 = sum - (nums1[i] - nums2[i]).abs() as i64
                + if diff1 < diff2 { diff1 } else { diff2 };
            if tmp < result {
                result = tmp;
            }
        }
        (result % modulo) as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                if nums1.len() == len {
                    nums2.push(num);
                    if nums2.len() == len {
                        break;
                    }
                } else {
                    nums1.push(num);
                }
            }
        }
    }

    if 0 == len || 2 * len != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Minimum absolute sum difference: {}",
        Solution::min_absolute_sum_diff(nums1, nums2)
    );
}
