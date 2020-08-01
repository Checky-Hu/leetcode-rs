use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut min: i32 = i32::max_value();
        let mut max: i32 = i32::min_value();
        let mut sum: i32 = 0;
        for v in salary.iter() {
            sum += *v;
            if *v > max {
                max = *v;
            }
            if *v < min {
                min = *v;
            }
        }
        (sum - max - min) as f64 / (salary.len() - 2) as f64
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut salary: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                salary.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Average: {}", Solution::average(salary));
}
