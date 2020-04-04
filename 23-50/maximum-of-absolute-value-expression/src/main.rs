use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let len: usize = arr1.len();
        let mut status: Vec<(i32, i32)> = vec![(i32::max_value(), i32::min_value()); 4];
        for i in 0..len {
            let t1: i32 = arr1[i] + arr2[i] - i as i32;
            if t1 < status[0].0 {
                status[0].0 = t1;
            }
            if t1 > status[0].1 {
                status[0].1 = t1;
            }
            let t2: i32 = arr1[i] - arr2[i] - i as i32;
            if t2 < status[1].0 {
                status[1].0 = t2;
            }
            if t2 > status[1].1 {
                status[1].1 = t2;
            }
            let t3: i32 = arr2[i] - arr1[i] - i as i32;
            if t3 < status[2].0 {
                status[2].0 = t3;
            }
            if t3 > status[2].1 {
                status[2].1 = t3;
            }
            let t4: i32 = arr1[i] + arr2[i] + i as i32;
            if t4 < status[3].0 {
                status[3].0 = t4;
            }
            if t4 > status[3].1 {
                status[3].1 = t4;
            }
        }
        let mut result: i32 = 0;
        for v in status.iter() {
            let t: i32 = v.1 - v.0;
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
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if arr1.len() == n as usize {
                    arr2.push(number);
                } else {
                    arr1.push(number);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Maximum absolute value expression: {}",
        Solution::max_abs_val_expr(arr1, arr2)
    );
}
