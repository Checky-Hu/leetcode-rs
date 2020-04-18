use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let len: usize = s.len();
        let s_bytes: &[u8] = s.as_bytes();
        let t_bytes: &[u8] = t.as_bytes();
        let mut deltas: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            let u1: u8 = s_bytes[i];
            let u2: u8 = t_bytes[i];
            deltas.push(if u1 >= u2 {
                (u1 - u2) as i32
            } else {
                (u2 - u1) as i32
            });
        }
        let mut result: usize = 0;
        let mut left: usize = 0;
        let mut sum: i32 = 0;
        for i in 0..len {
            sum += deltas[i];
            if sum > max_cost {
                let t: usize = if left < len { i - left } else { 0 };
                if t > result {
                    result = t;
                }
                while sum > max_cost {
                    sum -= deltas[left];
                    left += 1;
                }
            }
        }
        let t: usize = if left < len { len - left } else { 0 };
        if t > result {
            result = t;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut t: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            2 => t = arg,
            _ => {
                ret += 1;
                let max_cost: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Max equal substring: {}",
                    Solution::equal_substring(s, t, max_cost)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
    }
}
