use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let len: usize = people.len();
        let mut tmp: Vec<i32> = people;
        tmp.sort();
        let mut s: usize = 0;
        let mut e: usize = len - 1;
        let mut result: i32 = 0;
        while s < e {
            if tmp[s] + tmp[e] <= limit {
                s += 1;
            }
            result += 1;
            e -= 1;
        }
        if s == e {
            result += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut limit: i32 = 0;
    let mut people: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => limit = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                people.push(t);
            }
        }
    }

    if 0 == limit || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Number of rescue boats: {}",
        Solution::num_rescue_boats(people, limit)
    );
}
