use std::env;

struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let current: String = "./".to_string();
        let parent: String = "../".to_string();
        let mut result: Vec<String> = Vec::new();
        for log in logs {
            if log == current {
                continue;
            } else if log == parent {
                result.pop();
            } else {
                result.push(log);
            }
        }
        result.len() as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut logs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            logs.push(arg);
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }

    println!("Minimum operations: {}", Solution::min_operations(logs));
}
