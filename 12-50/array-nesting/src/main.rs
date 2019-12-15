use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut flags: Vec<bool> = vec![false; len];
        let mut result: i32 = 0;
        for i in 0..len {
            if !flags[i] {
                let mut set: HashSet<usize> = HashSet::new();
                let mut pre: usize = nums[i] as usize;
                loop {
                    if set.contains(&pre) {
                        break;
                    } else {
                        set.insert(pre);
                        flags[pre] = true;
                        pre = nums[pre] as usize;
                    }
                }
                let tmp: i32 = set.len() as i32;
                if tmp > result {
                    result = tmp;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Longest circle: {}", Solution::array_nesting(nums));
}
