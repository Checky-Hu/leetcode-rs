use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_times_all_blue(light: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut right: i32 = 0;
        for (i, v) in light.iter().enumerate() {
            if *v > right {
                right = *v;
            }
            if i as i32 + 1 == right {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut light: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                light.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    println!(
        "Times when all light blue: {}",
        Solution::num_times_all_blue(light)
    );
}
