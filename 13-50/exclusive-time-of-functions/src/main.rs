use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; n as usize];
        let mut stack: Vec<usize> = Vec::new();
        let mut pre_time: i32 = 0;
        for log in logs {
            let bytes: &[u8] = log.as_bytes();
            let len: usize = bytes.len();
            let mut id: usize = 0;
            let mut i: usize = 0;
            loop {
                if bytes[i] == 58 {
                    break;
                } else {
                    id = id * 10 + bytes[i] as usize - 48;
                    i += 1;
                }
            }
            let mut time: i32 = 0;
            let mut multi: i32 = 1;
            i = len - 1;
            loop {
                if bytes[i] == 58 {
                    break;
                } else {
                    time += (bytes[i] as i32 - 48) * multi;
                    multi *= 10;
                    i -= 1;
                }
            }
            let end: bool = bytes[i - 1] == 100;
            if let Some(x) = stack.last() {
                result[*x as usize] += time - pre_time;
            }
            pre_time = time;
            if !end {
                stack.push(id);
            } else if let Some(x) = stack.pop() {
                result[x] += 1;
                pre_time += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut logs: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let log: String = arg;
                logs.push(log);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::exclusive_time(n, logs);
    for t in result {
        print!("{} ", t);
    }
    println!();
}
