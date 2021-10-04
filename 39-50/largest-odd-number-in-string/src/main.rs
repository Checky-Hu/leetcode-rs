use std::env;

struct Solution {}

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        let len: usize = num.len();
        let mut result: usize = len;
        for (i, b) in num.bytes().enumerate() {
            if b & 1 == 1 {
                result = i;
            }
        }
        if result == len {
            String::new()
        } else {
            num[0..=result].to_string()
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: String = arg;
                println!("Largest odd number: {}", Solution::largest_odd_number(num));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
