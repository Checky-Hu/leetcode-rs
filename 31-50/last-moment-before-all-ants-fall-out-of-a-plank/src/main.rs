use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for v in left.iter() {
            if *v > result {
                result = *v;
            }
        }
        for v in right.iter() {
            let t: i32 = n - *v;
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if left.len() == k as usize {
                    right.push(t);
                } else {
                    left.push(t);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (2 + arg2) parameters.");
        return;
    }

    println!(
        "Lst moment before all ants fall out of a plank: {}",
        Solution::get_last_moment(n, left, right)
    );
}
