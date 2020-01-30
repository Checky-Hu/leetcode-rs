use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_subarray_bounded_max(a: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut pre_stop: i32 = -1;
        let mut flags: Vec<usize> = Vec::new();
        let mut result: usize = 0;
        for (i, n) in a.iter().enumerate() {
            if *n < l {
                continue;
            } else if l <= *n && *n <= r {
                flags.push(i);
            } else {
                let mut pre_start: usize = (pre_stop + 1) as usize;
                for pos in flags {
                    result += (i - pos) * (pos + 1 - pre_start);
                    pre_start = pos + 1;
                }
                flags = Vec::new();
                pre_stop = i as i32;
            }
        }
        let len: usize = a.len();
        let mut pre_start: usize = (pre_stop + 1) as usize;
        for pos in flags {
            result += (len - pos) * (pos + 1 - pre_start);
            pre_start = pos + 1;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut l: i32 = 0;
    let mut r: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => l = i32::from_str(&arg).expect("Error parse."),
            2 => r = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    println!(
        "Number of bounded max subarrays: {}",
        Solution::num_subarray_bounded_max(a, l, r)
    );
}
