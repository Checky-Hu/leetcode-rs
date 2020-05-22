use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let len: usize = start_time.len();
        let mut result: i32 = 0;
        for i in 0..len {
            if start_time[i] <= query_time && query_time <= end_time[i] {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut query_time: i32 = 0;
    let mut n: i32 = 0;
    let mut start_time: Vec<i32> = Vec::new();
    let mut end_time: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => query_time = i32::from_str(&arg).expect("Error parse."),
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let time: i32 = i32::from_str(&arg).expect("Error parse.");
                if start_time.len() == n as usize {
                    end_time.push(time);
                } else {
                    start_time.push(time);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (2 + 2 * arg2) parameters.");
        return;
    }

    println!(
        "Busy students: {}",
        Solution::busy_student(start_time, end_time, query_time)
    );
}
