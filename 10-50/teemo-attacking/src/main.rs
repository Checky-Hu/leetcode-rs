use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_positioned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let len: usize = time_series.len();
        let mut result: i32 = 0;
        let mut i: usize = 1;
        while i < len {
            if time_series[i] - time_series[i - 1] < duration {
                result += time_series[i] - time_series[i - 1];
            } else {
                result += duration;
            }
            i += 1;
        }
        if i == len {
            result += duration;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut duration: i32 = 0;
    let mut time_series: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => duration = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let time: i32 = i32::from_str(&arg).expect("Error parse.");
                time_series.push(time);
            },
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    println!("Positioned duration: {}", Solution::find_positioned_duration(time_series, duration));
}
