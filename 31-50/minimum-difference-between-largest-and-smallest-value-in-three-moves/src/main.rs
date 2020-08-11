use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn min_difference_loop(temp: &[i32], s: usize, e: usize, n: i32) -> i32 {
        if n == 0 {
            temp[e] - temp[s]
        } else {
            let result1: i32 = Solution::min_difference_loop(temp, s + 1, e, n - 1);
            let result2: i32 = Solution::min_difference_loop(temp, s, e - 1, n - 1);
            if result1 < result2 {
                result1
            } else {
                result2
            }
        }
    }

    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let len: usize = nums.len();
        if len <= 4 {
            return 0;
        }
        let mut temp: Vec<i32> = nums;
        temp.sort();
        Solution::min_difference_loop(&temp, 0, len - 1, 3)
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
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Minimum difference: {}", Solution::min_difference(nums));
}
