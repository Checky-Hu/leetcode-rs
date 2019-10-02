use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let mut t: usize = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        loop {
            slow = nums[slow] as usize;
            t = nums[t] as usize;
            if slow == t {
                break;
            }
        }
        slow as i32
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Duplicate number: {}", Solution::find_duplicate(nums));
}
