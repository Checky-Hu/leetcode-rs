use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        let mut result: i32 = 0;
        let mut prefix: Vec<i32> = Vec::new();
        for num in nums {
            if let Some(x) = prefix.pop() {
                if x >= num {
                    result += 1;
                    if result > 1 {
                        return false;
                    } else if let Some(y) = prefix.last() {
                        if *y >= num {
                            prefix.push(x);
                        } else {
                            prefix.push(num);
                        }
                    } else {
                        prefix.push(num);
                    }
                } else {
                    prefix.push(x);
                    prefix.push(num);
                }
            } else {
                prefix.push(num);
            }
        }
        true
    }
}

fn main() {
    let mut ret: usize = 0;
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

    println!("Can be increasing: {}", Solution::can_be_increasing(nums));
}
