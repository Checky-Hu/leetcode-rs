use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        let mut r1: i32 = 0;
        let mut r2: i32 = 0;
        for i in 0..len {
            let mut t: i32 = nums[i];
            if i > 0 && nums[i - 1] <= t {
                t = nums[i - 1] - 1;
            }
            if i + 1 < len && nums[i + 1] <= t {
                t = nums[i + 1] - 1;
            }
            if i & 1 == 0 {
                r1 += nums[i] - t;
            } else {
                r2 += nums[i] - t;
            }
        }
        if r1 < r2 {
            r1
        } else {
            r2
        }
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Moves to make zigzag: {}",
        Solution::moves_to_make_zigzag(nums)
    );
}
