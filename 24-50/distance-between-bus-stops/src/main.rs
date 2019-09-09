use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let len: usize = distance.len();
        let mut d1: i32 = 0;
        let mut d2: i32 = 0;
        let mut reach_des: bool = false;
        let mut i: usize = start as usize;
        loop {
            if reach_des {
                d2 += distance[i];
                if i == len - 1 {
                    i = 0;
                } else {
                    i += 1;
                }
                if i == start as usize {
                    break;
                }
            } else {
                d1 += distance[i];
                if i == len - 1 {
                    i = 0;
                } else {
                    i += 1;
                }
                if i == destination as usize {
                    reach_des = true;
                }
            }
        }
        if d1 > d2 {
            d2
        } else {
            d1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut start: i32 = 0;
    let mut destination: i32 = 0;
    let mut distance: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => start = i32::from_str(&arg).expect("Error parse."),
            2 => destination = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                distance.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Distance: {}", Solution::distance_between_bus_stops(distance, start, destination));
}
