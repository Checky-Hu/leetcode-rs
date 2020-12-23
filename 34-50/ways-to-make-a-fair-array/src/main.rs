use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        // (even, odd)
        let mut vec: Vec<(i32, i32)> = Vec::with_capacity(len + 1);
        vec.push((0, 0));
        for (i, n) in nums.iter().enumerate() {
            let cur: (i32, i32) = if i & 1 == 0 {
                (vec[i].0 + *n, vec[i].1)
            } else {
                (vec[i].0, vec[i].1 + *n)
            };
            vec.push(cur);
        }
        let mut result: i32 = 0;
        for i in 1..=len {
            if vec[i - 1].0 + vec[len].1 - vec[i].1 == vec[i - 1].1 + vec[len].0 - vec[i].0 {
                result += 1;
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Ways to make fair: {}", Solution::ways_to_make_fair(nums));
}
