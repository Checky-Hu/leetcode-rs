use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let step: i32 = (0 - target[0]).abs() + (0 - target[1]).abs();
        for ghost in &ghosts {
            let tmp: i32 = (target[0] - ghost[0]).abs() + (target[1] - ghost[1]).abs();
            if tmp <= step {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: Vec<i32> = Vec::new();
    let mut ghosts: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let v: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret <= 2 {
                    target.push(v);
                } else {
                    tmp.push(v);
                    if tmp.len() == 2 {
                        ghosts.push(tmp);
                        tmp = Vec::new();
                    }
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Escape ghosts: {}", Solution::escape_ghosts(ghosts, target));
}
