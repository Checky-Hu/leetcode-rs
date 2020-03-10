use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k == 2 || k == 5 {
            return -1;
        }
        let mut result: i32 = -1;
        let mut n: i32 = 1;
        let mut i: i32 = 1;
        let mut flags: Vec<bool> = vec![false; k as usize];
        loop {
            let t: i32 = n % k;
            if t == 0 {
                result = i;
                break;
            }
            if flags[t as usize] {
                break;
            } else {
                flags[t as usize] = true;
                n = t * 10 + 1;
                i += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Smallest integer divisible by {}: {}",
                k,
                Solution::smallest_repunit_div_by_k(k)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
