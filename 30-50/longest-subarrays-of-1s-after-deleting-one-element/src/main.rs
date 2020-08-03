use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_subarray_of_1: i32 = 0;
        let mut pre_subarray_of_1: i32 = 0;
        let mut cur_subarray_of_1: i32 = 0;
        let mut pre_subarray_of_0: i32 = 0;
        let mut cur_subarray_of_0: i32 = 0;
        let mut find_0: bool = false;
        let mut result: i32 = 0;
        for num in nums.iter() {
            if *num == 0 {
                find_0 = true;
                if cur_subarray_of_0 == 0 {
                    if pre_subarray_of_0 == 1 {
                        let t: i32 = pre_subarray_of_1 + cur_subarray_of_1;
                        if t > result {
                            result = t;
                        }
                    }
                    if cur_subarray_of_1 > max_subarray_of_1 {
                        max_subarray_of_1 = cur_subarray_of_1;
                    }
                    pre_subarray_of_1 = cur_subarray_of_1;
                    cur_subarray_of_1 = 0;
                }
                cur_subarray_of_0 += 1;
            } else {
                if cur_subarray_of_1 == 0 {
                    pre_subarray_of_0 = cur_subarray_of_0;
                    cur_subarray_of_0 = 0;
                }
                cur_subarray_of_1 += 1;
            }
        }
        if cur_subarray_of_0 == 0 {
            if pre_subarray_of_0 == 1 {
                let t: i32 = pre_subarray_of_1 + cur_subarray_of_1;
                if t > result {
                    result = t;
                }
            }
            if cur_subarray_of_1 > max_subarray_of_1 {
                max_subarray_of_1 = cur_subarray_of_1;
            }
        }
        if find_0 {
            if result > max_subarray_of_1 {
                result
            } else {
                max_subarray_of_1
            }
        } else {
            max_subarray_of_1 - 1
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
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Longest subarray: {}", Solution::longest_subarray(nums));
}
