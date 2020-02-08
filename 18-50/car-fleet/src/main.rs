extern crate quicksort;

use quicksort::qstuple;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let len: usize = position.len();
        if len == 0 {
            return 0;
        }
        let mut tuples: Vec<(i32, i32)> = Vec::with_capacity(len);
        for i in 0..len {
            tuples.push((position[i], speed[i]));
        }
        qstuple::quick_sort(&mut tuples, 0, len - 1);
        let mut i: usize = len - 1;
        let mut result: (f64, i32) = (0f64, 0i32);
        loop {
            let time: f64 = (target - tuples[i].0) as f64 / tuples[i].1 as f64;
            if result.0 < time {
                result.0 = time;
                result.1 += 1;
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result.1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: i32 = 0;
    let mut n: i32 = 0;
    let mut position: Vec<i32> = Vec::new();
    let mut speed: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let v: i32 = i32::from_str(&arg).expect("Error parse.");
                if position.len() as i32 == n {
                    speed.push(v);
                } else {
                    position.push(v);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (2 + 2 * arg2) parameters.");
        return;
    }

    println!(
        "Car fleets: {}",
        Solution::car_fleet(target, position, speed)
    );
}
