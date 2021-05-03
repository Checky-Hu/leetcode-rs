use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let len: usize = nums.len();
        let index: usize = start as usize;
        let mut i: usize = 0;
        loop {
            if i <= index && nums[index - i] == target {
                break;
            }
            if index + i < len && nums[index + i] == target {
                break;
            }
            i += 1;
        }
        i as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut target: i32 = 0;
    let mut start: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            2 => start = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!(
        "Minimum distance: {}",
        Solution::get_min_distance(nums, target, start)
    );
}
