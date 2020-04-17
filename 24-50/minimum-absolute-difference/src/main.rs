use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut tmp: Vec<i32> = arr;
        tmp.sort();
        let mut min: i32 = i32::max_value();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 1..tmp.len() {
            let t: i32 = tmp[i] - tmp[i - 1];
            if t < min {
                min = t;
                result.clear();
                result.push(vec![tmp[i - 1], tmp[i]]);
            } else if t == min {
                result.push(vec![tmp[i - 1], tmp[i]]);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::minimum_abs_difference(arr);
    for r in result.iter() {
        print!("[{}, {}]", r[0], r[1]);
    }
    println!();
}
