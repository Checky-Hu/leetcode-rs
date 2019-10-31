use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string()
        }
        let mut tmp: String = String::with_capacity(8);
        let mut n: i32 = num;
        for _i in 0..8 {
            let cur: u8 = (n & 15) as u8;
            if cur < 10 {
                tmp.push((cur + 48) as char);
            } else {
                tmp.push((cur + 87) as char);
            }
            n >>= 4;
        }
        let mut result: String = String::new();
        let mut is_zero: bool = true;
        loop {
            match tmp.pop() {
                Some(x) => {
                    if !is_zero || x != '0' {
                        is_zero = false;
                        result.push(x);
                    }
                },
                None => break,
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("{} to hex: {}", num, Solution::to_hex(num));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
