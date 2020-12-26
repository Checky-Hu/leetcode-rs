use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums.iter() {
            match map.get_mut(num) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*num, 1);
                }
            }
        }
        let mut set: HashSet<i32> = HashSet::new();
        let mut result: i32 = 0;
        for (key, val) in map.iter() {
            let t: i32 = k - *key;
            if !set.contains(&t) {
                if let Some(x) = map.get(&t) {
                    result += if t == *key {
                        *x >> 1
                    } else if *val > *x {
                        *x
                    } else {
                        *val
                    };
                }
                set.insert(*key);
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
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Maximum operations: {}", Solution::max_operations(nums, k));
}
