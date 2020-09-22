use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut sum: i32 = 0;
        for n in nums.iter() {
            sum = (sum + *n) % p;
        }
        if sum == 0 {
            return 0;
        }
        let mut tmp: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let len: i32 = nums.len() as i32;
        let mut result: i32 = len;
        for (i, n) in nums.iter().enumerate() {
            tmp = (tmp + *n) % p;
            let target: i32 = (tmp + p - sum) % p;
            if let Some(x) = map.get(&target) {
                let t: i32 = i as i32 - *x;
                if t < result {
                    result = t;
                }
            }
            map.insert(tmp, i as i32);
        }
        if result == len {
            -1
        } else {
            result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut p: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => p = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Minimum subarray for divisible by {}: {}",
        p,
        Solution::min_subarray(nums, p)
    );
}
