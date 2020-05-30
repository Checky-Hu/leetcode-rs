use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len: usize = card_points.len();
        let size: usize = len - k as usize;
        let mut sum: i32 = 0;
        let mut exclude: i32 = 0;
        let mut count: usize = 0;
        let mut result: i32 = i32::max_value();
        for (i, v) in card_points.iter().enumerate() {
            sum += *v;
            if count == size {
                exclude += *v - card_points[i - size];
            } else {
                exclude += *v;
                count += 1;
            }
            if count == size && exclude < result {
                result = exclude;
            }
        }
        sum - result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut card_points: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                card_points.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Maximum score: {}", Solution::max_score(card_points, k));
}
