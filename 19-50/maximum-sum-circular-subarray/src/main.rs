use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        let mut max1: i32 = i32::min_value();
        let mut min1: i32 = i32::max_value();
        let mut max2: i32 = i32::min_value();
        let mut min2: i32 = i32::max_value();
        let mut sum: i32 = 0;
        let mut all_negative: bool = true;
        let mut max_negative: i32 = i32::min_value();
        for v in a.iter() {
            if *v > 0 {
                all_negative = false;
            } else {
                if *v > max_negative {
                    max_negative = *v;
                }
            }
            sum += *v;
            if sum > max1 {
                max1 = sum;
            }
            if sum < min1 {
                min1 = sum;
            }
            if sum - min1 > max2 {
                max2 = sum - min1;
            }
            if sum - max1 < min2 {
                min2 = sum - max1;
            }
        }
        if all_negative {
            max_negative
        } else if max2 > sum - min2 {
            max2
        } else {
            sum - min2
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Max subarray sum: {}",
        Solution::max_subarray_sum_circular(a)
    );
}
