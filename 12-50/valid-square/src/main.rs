use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let dis1: i32 = (p1[0] - p2[0]) * (p1[0] - p2[0]) + (p1[1] - p2[1]) * (p1[1] - p2[1]);
        if dis1 == 0 {
            return false
        }
        let dis2: i32 = (p1[0] - p3[0]) * (p1[0] - p3[0]) + (p1[1] - p3[1]) * (p1[1] - p3[1]);
        if dis2 == 0 {
            return false
        }
        let dis3: i32 = (p1[0] - p4[0]) * (p1[0] - p4[0]) + (p1[1] - p4[1]) * (p1[1] - p4[1]);
        if dis3 == 0 {
            return false
        }
        if dis1 == dis2 && dis1 * 2 == dis3 {
            (p4[0] - p2[0]) * (p4[0] - p2[0]) + (p4[1] - p2[1]) * (p4[1] - p2[1]) ==
	    (p4[0] - p3[0]) * (p4[0] - p3[0]) + (p4[1] - p3[1]) * (p4[1] - p3[1])
        } else if dis1 == dis3 && dis1 * 2 == dis2 {
            (p3[0] - p2[0]) * (p3[0] - p2[0]) + (p3[1] - p2[1]) * (p3[1] - p2[1]) ==
	    (p3[0] - p4[0]) * (p3[0] - p4[0]) + (p3[1] - p4[1]) * (p3[1] - p4[1])
        } else if dis2 == dis3 && dis2 * 2 == dis1 {
            (p2[0] - p3[0]) * (p2[0] - p3[0]) + (p2[1] - p3[1]) * (p2[1] - p3[1]) ==
	    (p2[0] - p4[0]) * (p2[0] - p4[0]) + (p2[1] - p4[1]) * (p2[1] - p4[1])
        } else {
            false
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut input: Vec<i32> = Vec::with_capacity(8);
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                input.push(t);
                if ret == 8 {
                    break;
                }
            },
        }
    }

    if 8 > ret {
        println!("Require 8 parameters.");
        return
    }

    let p1: Vec<i32> = input.drain(0..2).collect();
    let p2: Vec<i32> = input.drain(0..2).collect();
    let p3: Vec<i32> = input.drain(0..2).collect();
    println!("Valid square: {}", Solution::valid_square(p1, p2, p3, input));
}
