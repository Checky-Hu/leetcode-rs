use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: i32 = 0;
        for v in nums.iter() {
            match map.get_mut(v) {
                Some(x) => {
                    if *x == 1 {
                        result -= *v;
                    }
                    *x += 1;
                }
                None => {
                    result += *v;
                    map.insert(*v, 1);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
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

    println!("Sum of unique elements: {}", Solution::sum_of_unique(nums));
}
