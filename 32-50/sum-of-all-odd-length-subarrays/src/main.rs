use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let len: usize = arr.len();
        let mut result: i32 = 0;
        let mut length: usize = 1;
        let mut prefix: i32 = 0;
        while length <= len {
            if prefix == 0 {
                prefix = arr[0];
            } else {
                prefix += arr[length - 2] + arr[length - 1];
            }
            result += prefix;
            let mut current: i32 = prefix;
            for i in 0..(len - length) {
                current += arr[i + length] - arr[i];
                result += current;
            }
            length += 2;
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
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Sum of all odd-length subarrays: {}",
        Solution::sum_odd_length_subarrays(arr)
    );
}
