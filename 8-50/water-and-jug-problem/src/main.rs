extern crate gcd;

use gcd::gcdi32;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        z == 0 || (x + y >= z && z % gcdi32::gcd(x, y) == 0)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => x = i32::from_str(&arg).expect("Error parse."),
            2 => y = i32::from_str(&arg).expect("Error parse."),
            3 => {
                ret = index;
                let z: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Can measure: {}", Solution::can_measure_water(x, y, z));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
    }
}
