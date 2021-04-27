use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let len: usize = nums.len();
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort();
        let mut ret: usize = 0;
        let mut sum: usize = 0;
        let mut s: usize = 0;
        let mut e: usize = 0;
        while e < len {
            sum += nums_mut[e] as usize;
            while sum + (k as usize) < (e - s + 1) * (nums_mut[e] as usize) {
                sum -= nums_mut[s] as usize;
                s += 1;
            }
            let t: usize = e - s + 1;
            if t > ret {
                ret = t;
            }
            e += 1;
        }
        ret as i32
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
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Max frequency: {}", Solution::max_frequency(nums, k));
}
