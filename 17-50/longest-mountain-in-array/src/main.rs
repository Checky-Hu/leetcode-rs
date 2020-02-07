use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let len: usize = a.len();
        let mut pre_data: (usize, bool, bool) = (0, false, false);
        for i in 1..len {
            match a[i].cmp(&a[i - 1]) {
                Ordering::Less => {
                    if pre_data.1 {
                        pre_data.2 = true;
                    }
                }
                Ordering::Equal => {
                    if pre_data.1 && pre_data.2 {
                        let tmp: i32 = (i - pre_data.0) as i32;
                        if tmp > result {
                            result = tmp;
                        }
                    }
                    pre_data.1 = false;
                    pre_data.2 = false;
                }
                Ordering::Greater => {
                    if pre_data.1 {
                        if pre_data.2 {
                            let tmp: i32 = (i - pre_data.0) as i32;
                            if tmp > result {
                                result = tmp;
                            }
                            pre_data = (i - 1, true, false);
                        }
                    } else {
                        pre_data.0 = i - 1;
                        pre_data.1 = true;
                    }
                }
            }
        }
        if pre_data.1 && pre_data.2 {
            let tmp: i32 = (len - pre_data.0) as i32;
            if tmp > result {
                result = tmp;
            }
        }
        result
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

    println!("Longest mountain: {}", Solution::longest_mountain(a));
}
