use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut max: i32 = i32::min_value();
        for n in candies.iter() {
            if max < *n {
                max = *n;
            }
        }
        let mut result: Vec<bool> = Vec::with_capacity(candies.len());
        for n in candies.iter() {
            if *n + extra_candies >= max {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut extra_candies: i32 = 0;
    let mut candies: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => extra_candies = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                candies.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<bool> = Solution::kids_with_candies(candies, extra_candies);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
