use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let len: usize = arr.len();
        // (number, compare)
        let mut pre: (i32, i32) = (arr[0], 0);
        for v in arr.iter().take(len).skip(1) {
            if *v > pre.0 {
                pre.0 = *v;
                pre.1 = 1;
            } else {
                pre.1 += 1;
            }
            if pre.1 == k {
                return pre.0;
            }
        }
        pre.0
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

    println!("Winner of an array game: {}", Solution::get_winner(arr, k));
}
