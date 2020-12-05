use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr_mut: Vec<i32> = arr;
        arr_mut.sort_unstable();
        let len: usize = arr_mut.len();
        let mut sum: f64 = 0_f64;
        for v in arr_mut.iter().take(len * 19 / 20).skip(len / 20) {
            sum += *v as f64;
        }
        sum / ((len * 9 / 10) as f64)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Mean of array: {}", Solution::trim_mean(arr));
}
