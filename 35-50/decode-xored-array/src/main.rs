use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut prefix: i32 = first;
        result.push(prefix);
        for v in encoded.iter() {
            let t: i32 = *v ^ prefix;
            result.push(t);
            prefix = t;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut first: i32 = 0;
    let mut encoded: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => first = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                encoded.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result = Solution::decode(encoded, first);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
