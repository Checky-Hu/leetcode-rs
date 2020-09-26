use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        let total: usize = len1 + len2;
        let target: usize = (total >> 1) + 1;
        let mut array: Vec<i32> = Vec::with_capacity(target);
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        for _i in 0..target {
            let use1: bool = if i1 == len1 {
                false
            } else if i2 == len2 {
                true
            } else {
                nums1[i1] < nums2[i2]
            };
            if use1 {
                array.push(nums1[i1]);
                i1 += 1;
            } else {
                array.push(nums2[i2]);
                i2 += 1;
            }
        }
        if total & 1 == 0 {
            (array[target - 1] as f64 + array[target - 2] as f64) / 2_f64
        } else {
            array[target - 1] as f64
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut count: usize = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => count = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                if nums1.len() == count {
                    nums2.push(num);
                } else {
                    nums1.push(num);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Median of two sorted arrays: {}",
        Solution::find_median_sorted_arrays(nums1, nums2)
    );
}
