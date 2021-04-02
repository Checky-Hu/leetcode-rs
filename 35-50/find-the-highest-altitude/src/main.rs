use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut position: i32 = 0;
        let mut result: i32 = 0;
        for v in gain.iter() {
            position += *v;
            if position > result {
                result = position;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut gain: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                gain.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Largest altitude: {}", Solution::largest_altitude(gain));
}
