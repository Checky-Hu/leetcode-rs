use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum: i32 = 0;
        let mut result: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        for n in nums {
            sum += n;
            if let Some(x) = map.get(&(sum - k)) {
                result += *x;
            }
            match map.get_mut(&sum) {
                Some(x) => *x += 1,
                None => {
                    map.insert(sum, 1);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Subarray sum equals to {}: {}",
        k,
        Solution::subarray_sum(nums, k)
    );
}
