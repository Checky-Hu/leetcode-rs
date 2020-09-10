use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // (start, end) of range.
        let mut range: (i32, i32) = (-1, -2);
        // (begin, last) of negative number.
        let mut index: (i32, i32) = (-1, -1);
        let mut count: i32 = 0;
        let mut result: i32 = 0;
        for (i, num) in nums.iter().enumerate() {
            if *num == 0 {
                let t: i32 = if count & 1 == 0 {
                    range.1 - range.0 + 1
                } else {
                    let t1: i32 = index.1 - range.0;
                    let t2: i32 = range.1 - index.0;
                    if t1 > t2 {
                        t1
                    } else {
                        t2
                    }
                };
                if t > result {
                    result = t;
                }
                range.0 = -1;
                range.1 = -2;
                index.0 = -1;
                index.1 = -1;
                count = 0;
            } else {
                if range.0 < 0 {
                    range.0 = i as i32;
                }
                range.1 = i as i32;
                if *num < 0 {
                    count += 1;
                    if index.0 < 0 {
                        index.0 = i as i32;
                    }
                    index.1 = i as i32;
                }
            }
        }
        let t: i32 = if count & 1 == 0 {
            range.1 - range.0 + 1
        } else {
            let t1: i32 = index.1 - range.0;
            let t2: i32 = range.1 - index.0;
            if t1 > t2 {
                t1
            } else {
                t2
            }
        };
        if t > result {
            result = t;
        }
        result
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
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Maximum length of subarray with positive product: {}",
        Solution::get_max_len(nums)
    );
}
