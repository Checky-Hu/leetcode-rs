use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let len: usize = nums.len();
        let mut map: HashMap<i64, usize> = HashMap::new();
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < len {
            if i - j > k as usize {
                map.remove(&(nums[j] as i64));
                j += 1;
            }
            for key in map.keys() {
                if (*key <= nums[i] as i64 && nums[i] as i64 - *key <= t as i64) ||
                    (*key > nums[i] as i64 && *key - nums[i] as i64 <= t as i64) {
                    return true
                }
            }
            map.insert(nums[i].into(), i);
            i += 1;
        }
        false
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => t = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Contains: {}", Solution::contains_nearby_almost_duplicate(nums, k, t));
}
