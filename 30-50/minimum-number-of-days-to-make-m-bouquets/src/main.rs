use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn check_valid(vec: &[i32], m: i32, k: i32, mid: i32) -> bool {
        let mut result: i32 = 0;
        let mut current: i32 = 0;
        for v in vec.iter() {
            if *v <= mid {
                current += 1;
                if current == k {
                    result += 1;
                    current = 0;
                }
            } else {
                current = 0;
            }
        }
        result >= m
    }

    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut s: i32 = 1;
        let mut e: i32 = 1_000_000_000;
        let mut is_valid: bool = false;
        while s <= e {
            let mid: i32 = s + (e - s) / 2;
            if Solution::check_valid(&bloom_day, m, k, mid) {
                e = mid - 1;
                is_valid = true;
            } else {
                s = mid + 1;
            }
        }
        if is_valid {
            s
        } else {
            -1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: i32 = 0;
    let mut k: i32 = 0;
    let mut bloom_day: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                bloom_day.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Minimum number of days to make {} bouquets: {}",
        m,
        Solution::min_days(bloom_day, m, k)
    );
}
