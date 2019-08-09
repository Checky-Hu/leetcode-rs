use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn repeat_n_times(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut i: usize = 0;
        while i < len - 1 {
            let mut j: usize = 1;
            while j < 4 && i + j < len {
                if a[i] == a[i + j] {
                    return a[i]
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("N times: {}", Solution::repeat_n_times(a));
}

