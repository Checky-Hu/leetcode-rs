use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut result: i32 = 0;
        let mut sum: i32 = 0;
        for i in 0..nums.len() {
            sum += if nums[i] == 1 {
                1
            } else {
                -1
            };
            match map.get(&sum) {
                Some(x) => {
                    let t: i32 = i as i32 - x;
                    if t > result {
                        result = t;
                    }
                },
                None => {
                    map.insert(sum, i as i32);
                },
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Max length: {}", Solution::find_max_length(nums));
}
