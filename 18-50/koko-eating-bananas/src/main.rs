use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut sum: i64 = 0;
        for p in piles.iter() {
            sum += *p as i64;
        }
        let mut result: i64 = sum / h as i64;
        if result == 0 {
            return 1;
        }
        loop {
            let mut count: i64 = 0;
            for p in piles.iter() {
                count += (*p as i64 + result - 1) / result;
            }
            if count <= h as i64 {
                break;
            } else {
                result += 1;
            }
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut h: i32 = 0;
    let mut piles: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => h = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                piles.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Minimum eating speed: {}",
        Solution::min_eating_speed(piles, h)
    );
}
