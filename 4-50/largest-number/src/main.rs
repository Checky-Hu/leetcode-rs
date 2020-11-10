use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let len: usize = nums.len();
        let mut strs: Vec<String> = Vec::with_capacity(len);
        for num in nums {
            strs.push(num.to_string());
        }
        strs.sort_by(|a, b| {
            let a_bytes: &[u8] = a.as_bytes();
            let a_len: usize = a_bytes.len();
            let b_bytes: &[u8] = b.as_bytes();
            let b_len: usize = b_bytes.len();
            let mut a_i: usize = 0;
            let mut b_i: usize = 0;
            loop {
                match a_bytes[a_i].cmp(&b_bytes[b_i]) {
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Greater => return Ordering::Less,
                    Ordering::Equal => {
                        if a_i + 1 == a_len {
                            if b_i + 1 == b_len {
                                break;
                            } else {
                                a_i = 0;
                                b_i += 1;
                            }
                        } else if b_i + 1 == b_len {
                            a_i += 1;
                            b_i = 0;
                        } else {
                            a_i += 1;
                            b_i += 1;
                        }
                    }
                }
            }
            Ordering::Equal
        });
        let flag: String = "0".to_string();
        if strs[0] == flag {
            flag
        } else {
            let mut result: String = String::new();
            for s in strs {
                result.push_str(&s);
            }
            result
        }
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

    println!("Largest number: {}", Solution::largest_number(nums));
}
