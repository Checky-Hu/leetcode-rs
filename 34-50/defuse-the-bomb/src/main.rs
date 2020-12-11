use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut prefix: i32 = 0;
        let len: usize = code.len();
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            result.push(match 0.cmp(&k) {
                Ordering::Less => {
                    if prefix == 0 {
                        for v in code.iter().take(k as usize + 1).skip(1) {
                            prefix += *v;
                        }
                    } else {
                        prefix += code[(i + k as usize) % len] - code[i];
                    }
                    prefix
                }
                Ordering::Equal => 0,
                Ordering::Greater => {
                    if prefix == 0 {
                        for v in code.iter().take(len).skip((len as i32 + k) as usize) {
                            prefix += *v;
                        }
                    } else {
                        prefix += code[i - 1] - code[(((i + len) as i32 + k - 1) as usize) % len];
                    }
                    prefix
                }
            });
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut code: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                code.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::decrypt(code, k);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
