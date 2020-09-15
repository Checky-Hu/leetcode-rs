use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut map: HashMap<i64, i32> = HashMap::new();
        let len1: usize = nums1.len();
        let len2: usize = nums2.len();
        let mut result: i32 = 0;
        for i in 0..(len2 - 1) {
            for j in (i + 1)..len2 {
                let t: i64 = (nums2[i] as i64) * (nums2[j] as i64);
                match map.get_mut(&t) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(t, 1);
                    }
                }
            }
        }
        for n in nums1.iter() {
            if let Some(x) = map.get(&((*n as i64) * (*n as i64))) {
                result += *x;
            }
        }
        map.clear();
        for i in 0..(len1 - 1) {
            for j in (i + 1)..len1 {
                let t: i64 = (nums1[i] as i64) * (nums1[j] as i64);
                match map.get_mut(&t) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(t, 1);
                    }
                }
            }
        }
        for n in nums2.iter() {
            if let Some(x) = map.get(&((*n as i64) * (*n as i64))) {
                result += *x;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut t: i32 = 0;
    let mut nums1: Vec<i32> = Vec::new();
    let mut nums2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => t = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if nums1.len() == t as usize {
                    nums2.push(n);
                } else {
                    nums1.push(n);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Number of triplets: {}",
        Solution::num_triplets(nums1, nums2)
    );
}
