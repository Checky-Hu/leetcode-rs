use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        // (sum, index)
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut sum: i32 = 0;
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            sum += *num;
            if let Some(x) = map.get(&(sum - target)) {
                ranges.push((*x + 1, i as i32));
            }
            map.insert(sum, i as i32);
        }
        let mut result: i32 = 0;
        let mut last: i32 = -1;
        for range in ranges.iter() {
            if range.0 > last {
                result += 1;
                last = range.1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
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

    println!(
        "Maximum number of non-overlapping subarrays: {}",
        Solution::max_non_overlapping(nums, target)
    );
}
