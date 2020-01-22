use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let start: String = "0000".to_string();
        let change: Vec<i32> = vec![-1, 1];
        let mut set: HashSet<String> = HashSet::new();
        for deadend in deadends {
            set.insert(deadend);
        }
        if set.contains(&start) {
            return -1;
        }
        let mut result: i32 = 0;
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.clone());
        let mut queue: Vec<String> = vec![start];
        while !queue.is_empty() {
            result += 1;
            let mut tmp: Vec<String> = Vec::new();
            for s in queue {
                let mut bytes: Vec<u8> = s.into_bytes();
                for i in 0..4 {
                    let cur_b: u8 = bytes[i];
                    for j in &change {
                        bytes[i] = ((bytes[i] as i32 - 48 + 10 + *j) % 10 + 48) as u8;
                        let cur_s: String = String::from_utf8(bytes.clone()).unwrap();
                        if cur_s == target {
                            return result;
                        }
                        if !visited.contains(&cur_s) && !set.contains(&cur_s) {
                            tmp.push(cur_s.clone());
                        }
                        visited.insert(cur_s);
                        bytes[i] = cur_b;
                    }
                }
            }
            queue = tmp;
        }
        -1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: String = String::new();
    let mut deadends: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                deadends.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Step to open lock: {}",
        Solution::open_lock(deadends, target)
    );
}
