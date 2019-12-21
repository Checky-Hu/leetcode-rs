use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts: Vec<i32> = vec![0; 26];
        let len: i32 = tasks.len() as i32;
        for task in tasks {
            counts[(task as u8 - 65) as usize] += 1;
        }
        let mut max: i32 = 0;
        let mut max_c: i32 = 1;
        for count in counts {
            if count > max {
                max = count;
                max_c = 1;
            } else if count == max {
                max_c += 1;
            }
        }
        let base: i32 = max * (n + 1);
        if len >= base {
            len
        } else {
            (max - 1) * (n + 1) + max_c
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = -1;
    let mut tasks: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let task: char = arg.chars().next().unwrap();
                tasks.push(task);
            },
        }
    }

    if 0 > n || 0 == ret {
        println!("Require at least two parameters.");
        return
    }

    println!("Least intervals: {}", Solution::least_interval(tasks, n));
}

