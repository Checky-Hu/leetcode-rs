use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut counts: Vec<i32> = vec![0; 46];
        for v in low_limit..=high_limit {
            let mut tmp: usize = v as usize;
            let mut pos: usize = 0;
            loop {
                pos += tmp % 10;
                tmp /= 10;
                if tmp == 0 {
                    break;
                }
            }
            counts[pos] += 1;
        }
        let mut result: i32 = 0;
        for v in counts.iter() {
            if *v > result {
                result = *v;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut low_limit: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => low_limit = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let high_limit: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Maximum number of balls in a box: {}",
                    Solution::count_balls(low_limit, high_limit)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
