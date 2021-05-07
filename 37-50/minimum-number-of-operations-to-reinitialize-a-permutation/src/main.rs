use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let len: usize = n as usize;
        let mut perm: Vec<i32> = Vec::with_capacity(len);
        for i in 0..n {
            perm.push(i);
        }
        let mut result: i32 = 1;
        loop {
            let mut temp: Vec<i32> = Vec::with_capacity(len);
            let mut is_same: bool = true;
            for i in 0..len {
                let t: i32 = if i & 1 == 0 {
                    perm[i >> 1]
                } else {
                    perm[(len + i - 1) >> 1]
                };
                if t != i as i32 {
                    is_same = false;
                }
                temp.push(t);
            }
            if is_same {
                break;
            } else {
                perm = temp;
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (i, arg) in env::args().enumerate() {
        match i {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Minimum operations: {}",
                    Solution::reinitialize_permutation(n)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
