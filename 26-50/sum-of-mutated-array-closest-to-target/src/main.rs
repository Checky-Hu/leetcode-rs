use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut max: i32 = 0;
        for n in arr.iter() {
            if *n > max {
                max = *n;
            }
        }
        let mut result: Vec<(i32, i32)> = vec![(0, 0), (max, 0)];
        let mut left: i32 = 0;
        let mut right: i32 = max;
        while left <= right {
            let mid: i32 = (right - left) / 2 + left;
            let mut sum: i32 = 0;
            for n in arr.iter() {
                sum += if *n > mid { mid } else { *n };
            }
            if sum > target {
                right = mid - 1;
                if mid < result[1].0 {
                    result[1].0 = mid;
                    result[1].1 = sum;
                }
            } else {
                left = mid + 1;
                if mid > result[0].0 {
                    result[0].0 = mid;
                    result[0].1 = sum;
                }
            }
        }
        if target - result[0].1 <= result[1].1 - target {
            result[0].0
        } else {
            result[1].0
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut target: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }

    println!("Best value: {}", Solution::find_best_value(arr, target));
}
