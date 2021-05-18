use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        let mut index1: usize = 0;
        let mut index2: usize = 0;
        let mut result: usize = 0;
        while index1 < len1 && index1 + result + 1 < len2 {
            let mut found_bigger: bool = false;
            while index2 < len2 {
                if nums1[index1] <= nums2[index2] {
                    found_bigger = true;
                    index2 += 1;
                } else {
                    break;
                }
            }
            if index2 == len2 {
                if found_bigger {
                    let t: usize = index2 - 1 - index1;
                    if t > result {
                        result = t;
                    }
                }
                break;
            } else if index1 == index2 {
                index1 += 1;
                index2 += 1;
            } else {
                let t: usize = index2 - 1 - index1;
                if t > result {
                    result = t;
                }
                index1 += 1;
            }
        }
        result as i32
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
                } else {
                    nums1.push(num);
                }
            }
        }
    }

    if 0 == len || len >= ret {
        println!("Require at least (1 + arg + 1) parameters.");
        return;
    }

    println!(
        "Maximum distance between a pair of values: {}",
        Solution::max_distance(nums1, nums2)
    );
}
