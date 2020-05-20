use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let push: String = "Push".to_string();
        let pop: String = "Pop".to_string();
        let len: usize = target.len();
        let mut i: usize = 0;
        let mut cur: i32 = 1;
        let mut result: Vec<String> = Vec::with_capacity(n as usize * 2);
        while i < len && cur <= n {
            result.push(push.clone());
            if target[i] == cur {
                i += 1;
            } else {
                result.push(pop.clone());
            }
            cur += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut target: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                target.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<String> = Solution::build_array(target, n);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
