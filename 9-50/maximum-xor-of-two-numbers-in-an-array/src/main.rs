use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut mask: i32 = 0;
        let mut i: i32 = 31;
        while i >= 0 {
            mask |= 1 << i;
            let mut set: HashSet<i32> = HashSet::new();
            for n in &nums {
                set.insert(n & mask);
            }
            let tmp: i32 = result | (1 << i);
            for v in set.iter() {
                if set.contains(&(tmp ^ v)) {
                    result = tmp;
                    break;
                }
            }
            i -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Maximum xor: {}", Solution::find_maximum_xor(nums));
}
