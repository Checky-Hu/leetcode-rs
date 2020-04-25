use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let len: usize = 1 << n;
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for i in 0..len {
            result.push((i ^ (i >> 1)) as i32);
        }
        loop {
            if let Some(x) = result.pop() {
                result.insert(0, x);
                if x == start {
                    break;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let start: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::circular_permutation(n, start);
                for r in result.iter() {
                    print!("{} ", *r);
                }
                println!();
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
