use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn mem_leak(memory1: i32, memory2: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; 3];
        result[1] = memory1;
        result[2] = memory2;
        let mut leak: i32 = 1;
        loop {
            if result[1] >= result[2] {
                if result[1] >= leak {
                    result[1] -= leak;
                    leak += 1;
                } else {
                    result[0] = leak;
                    break;
                }
            } else if result[2] >= leak {
                result[2] -= leak;
                leak += 1;
            } else {
                result[0] = leak;
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut memory1: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => memory1 = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let memory2: i32 = i32::from_str(&arg).expect("Error parse.");
                let result = Solution::mem_leak(memory1, memory2);
                println!(
                    "Crash status: ({}, {}, {})",
                    result[0], result[1], result[2]
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
