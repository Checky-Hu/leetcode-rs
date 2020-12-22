use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut result: Vec<u8> = Vec::with_capacity(n as usize);
        let mut i: i32 = n;
        let mut rest: i32 = k;
        while rest != 0 {
            let max: i32 = (i - 1) * 26;
            if max + 1 >= rest {
                result.push(97_u8);
                rest -= 1;
            } else {
                let cur: i32 = rest - max;
                result.push(cur as u8 + 96);
                rest -= cur;
            }
            i -= 1;
        }
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Smallest string: {}", Solution::get_smallest_string(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
