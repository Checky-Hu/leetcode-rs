use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = new_interval;
        for interval in intervals {
            if current.is_empty() || interval[1] < current[0] {
                results.push(interval);
            } else if interval[0] > current[1] {
                results.push(current);
                current = Vec::new();
                results.push(interval);
            } else {
                current[0] = std::cmp::min(interval[0], current[0]);
                current[1] = std::cmp::max(interval[1], current[1]);
            }
        }
        if !current.is_empty() {
            results.push(current);
        }
        results
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut new_interval: Vec<i32> = Vec::new();
    let mut intervals: Vec<Vec<i32>> = Vec::new();
    let mut interval: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if new_interval.len() == 2 {
                    interval.push(n);
                    if interval.len() == 2 {
                        intervals.push(interval);
                        interval = Vec::new();
                    }
                } else {
                    new_interval.push(n);
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::insert(intervals, new_interval);
    for r in result.iter() {
        print!("[{}, {}] ", r[0], r[1]);
    }
    println!();
}
