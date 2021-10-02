use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut result: bool;
        for num in left..=right {
            result = false;
            for range in ranges.iter() {
                if range[0] <= num && num <= range[1] {
                    result = true;
                    break;
                }
            }
            if !result {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut range: Vec<i32> = Vec::new();
    let mut ranges: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => left = i32::from_str(&arg).expect("Error parse."),
            2 => right = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                range.push(num);
                if range.len() == 2 {
                    ranges.push(range);
                    range = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !range.is_empty() {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "All integers is covered: {}",
        Solution::is_covered(ranges, left, right)
    );
}
