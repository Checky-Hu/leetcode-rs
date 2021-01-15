use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut vec: Vec<char> = Vec::new();
        let mut result: i32 = 0;
        let (high, low, prefix, suffix) = if x > y {
            (x, y, 'a', 'b')
        } else {
            (y, x, 'b', 'a')
        };
        let mut prefix_c: i32 = 0;
        let mut suffix_c: i32 = 0;
        for c in s.chars() {
            if c == suffix {
                if let Some(x) = vec.last() {
                    if *x == prefix {
                        vec.pop();
                        result += high;
                    } else {
                        vec.push(c);
                    }
                } else {
                    vec.push(c);
                }
            } else if c == prefix {
                vec.push(c);
            } else {
                while let Some(x) = vec.pop() {
                    if x == prefix {
                        prefix_c += 1;
                    } else {
                        suffix_c += 1;
                    }
                }
                result += if prefix_c < suffix_c {
                    prefix_c
                } else {
                    suffix_c
                } * low;
                prefix_c = 0;
                suffix_c = 0;
            }
        }
        while let Some(x) = vec.pop() {
            if x == prefix {
                prefix_c += 1;
            } else {
                suffix_c += 1;
            }
        }
        result += if prefix_c < suffix_c {
            prefix_c
        } else {
            suffix_c
        } * low;
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut x: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            2 => x = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let y: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Maximum gain: {}", Solution::maximum_gain(s, x, y));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
