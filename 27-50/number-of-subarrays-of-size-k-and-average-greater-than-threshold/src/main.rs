use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let target: i32 = k * threshold;
        let mut counts: i32 = 0;
        let mut sum: i32 = 0;
        let mut result: i32 = 0;
        for i in 0..arr.len() {
            if counts == k {
                sum = sum - arr[i - k as usize] + arr[i];
            } else {
                counts += 1;
                sum += arr[i];
            }
            if counts == k && sum >= target {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut threshold: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => threshold = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }

    println!(
        "Number of subarrays: {}",
        Solution::num_of_subarrays(arr, k, threshold)
    );
}
