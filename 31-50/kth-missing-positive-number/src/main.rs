use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut n: i32 = 0;
        let mut pre: i32 = 0;
        for num in arr.iter() {
            let t: i32 = n + *num - pre - 1;
            if t >= k {
                break;
            } else {
                pre = *num;
                n = t;
            }
        }
        pre + k - n
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "{}th missing positive: {}",
        k,
        Solution::find_kth_positive(arr, k)
    );
}
