use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let result: f64 = 5.5_f64 * minutes as f64 - (hour % 12) as f64 * 30_f64;
        if result > 0_f64 {
            if result > 180_f64 {
                360_f64 - result
            } else {
                result
            }
        } else if result < -180_f64 {
            360_f64 + result
        } else {
            0_f64 - result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut hour: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => hour = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let minutes: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Angle of clock: {}", Solution::angle_clock(hour, minutes));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
