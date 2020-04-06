use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut multi: i32 = 1;
        loop {
            let t: i32 = multi * 10;
            if t > num {
                break;
            } else {
                multi = t;
            }
        }
        let mut tmp: i32 = num;
        while tmp > 0 {
            if tmp / multi == 6 {
                return num + multi * 3;
            } else {
                tmp %= multi;
                multi /= 10;
            }
        }
        num
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Maximum 69 number: {}", Solution::maximum69_number(num));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
