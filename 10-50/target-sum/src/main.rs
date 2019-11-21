use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut dp: HashMap<i32, i32> = HashMap::new();
        dp.insert(0, 1);
        for n in nums {
            let mut tmp: HashMap<i32, i32> = HashMap::new();
            for (k, v) in dp.iter() {
                match tmp.get_mut(&(k + n)) {
                    Some(x) => *x += v,
                    None => {
                        tmp.insert(k + n, *v);
                    },
                };
                match tmp.get_mut(&(k - n)) {
                    Some(x) => *x += v,
                    None => {
                        tmp.insert(k - n, *v);
                    },
                };
            }
            dp = tmp;
        }
        match dp.get(&s) {
            Some(x) => *x,
            None => 0,
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    println!("Sum ways: {}", Solution::find_target_sum_ways(nums, s));
}
