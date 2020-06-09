use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut fir: i32 = i32::min_value();
        let mut sec: i32 = i32::min_value();
        for n in nums.iter() {
            if *n > fir {
                sec = fir;
                fir = *n;
            } else if *n > sec {
                sec = *n;
            }
        }
        (fir - 1) * (sec - 1)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Maximum product: {}", Solution::max_product(nums));
}
