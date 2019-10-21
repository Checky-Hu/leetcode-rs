use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        let len: usize = if k as usize <= len1 * len2 {
            k as usize
        } else {
            len1 * len2
        };
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(len);
        let mut index: Vec<usize> = vec![0; len1];
        for _i in 0..len {
            let mut cur: usize = 0;
            let mut sum: i32 = i32::max_value();
            for j in 0..len1 {
                if index[j] < len2 && sum > nums1[j] + nums2[index[j]] {
                    cur = j;
                    sum = nums1[j] + nums2[index[j]];
                }
            }
            result.push(vec![nums1[cur], nums2[index[cur]]]);
            index[cur] += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut len1: i32 = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => len1 = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret > len1 {
                    nums2.push(num);
                } else {
                    nums1.push(num);
                }
            },
        }
    }

    if 0 == ret || len1 == 0 || len1 >= ret {
        println!("Require at least (2 + len1 + len2) parameters.");
    }

    let result: Vec<Vec<i32>> = Solution::k_smallest_pairs(nums1, nums2, k);
    for r in result {
        println!("[{}, {}]", r[0], r[1]);
    }
}
