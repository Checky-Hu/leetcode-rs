use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut flags: Vec<i32> = vec![0; 60];
        for t in time {
            flags[(t % 60) as usize] += 1;
        }
        let mut result: i32 = flags[0] * (flags[0] - 1) / 2;
        result += flags[30] * (flags[30] - 1) / 2;
        let mut s: usize = 1;
        let mut e: usize = 59;
        while s < e {
            result += flags[s] * flags[e];
            s += 1;
            e -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut time: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                time.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Pairs: {}", Solution::num_pairs_divisible_by60(time));
}

