use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_sum(middle: i64, less: i64, greater: i64) -> i64 {
        if middle < less {
            (middle - 2) * middle + less + greater
        } else if middle < greater {
            middle * less - (less - 1) * less / 2 + (middle + 1) * middle / 2 - middle + greater
                - middle
        } else {
            middle * less - (less - 1) * less / 2 + middle * greater
                - (greater - 1) * greater / 2
                - middle
        }
    }

    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let len3: i64 = i64::from(max_sum);
        let len1: i64 = i64::from(index + 1);
        let len2: i64 = i64::from(n - index);
        let (less, greater) = if len1 < len2 {
            (len1, len2)
        } else {
            (len2, len1)
        };
        let mut left: i64 = 1;
        let mut right: i64 = len3;
        while left < right {
            let middle: i64 = (right - left) / 2 + left;
            let sum: i64 = Solution::count_sum(middle, less, greater);
            if sum <= len3 {
                if Solution::count_sum(middle + 1, less, greater) <= len3 {
                    left = middle + 1;
                } else {
                    left = middle;
                    break;
                }
            } else {
                right = middle - 1;
            }
        }
        left as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut index: i32 = 0;
    for (i, arg) in env::args().enumerate() {
        match i {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => index = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let max_sum: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Maximum value: {}", Solution::max_value(n, index, max_sum));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
