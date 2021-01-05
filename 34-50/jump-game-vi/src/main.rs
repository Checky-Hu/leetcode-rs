use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let dis: usize = k as usize;
        let len: usize = nums.len();
        let mut result: Vec<i32> = Vec::with_capacity(len);
        let mut index: Vec<usize> = Vec::with_capacity(dis);
        for i in 0..len {
            if i == 0 {
                result.push(nums[len - 1]);
                index.push(0);
            } else {
                result.push(nums[len - 1 - i] + result[index[0]]);
                while let Some(x) = index.last() {
                    if result[i] >= result[*x] {
                        index.pop();
                    } else {
                        break;
                    }
                }
                index.push(i);
                if i - index[0] >= dis {
                    index.remove(0);
                }
            }
        }
        result[len - 1]
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
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Maximum scores: {}", Solution::max_result(nums, k));
}
