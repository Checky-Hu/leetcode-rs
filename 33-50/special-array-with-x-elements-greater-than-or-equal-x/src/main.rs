use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums_mut: Vec<i32> = nums;
        nums_mut.sort_unstable();
        let len: usize = nums_mut.len();
        let mut prefix: i32 = 0;
        let mut x: i32 = len as i32;
        for v in nums_mut.iter() {
            if *v >= x && x > prefix {
                return x;
            } else {
                x -= 1;
                prefix = *v;
            }
        }
        -1
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
        println!("Require at least 2 parameters.");
        return;
    }

    println!("X of special array: {}", Solution::special_array(nums));
}
