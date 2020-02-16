use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut sum_a: i32 = 0;
        let mut set_a: HashSet<i32> = HashSet::new();
        for v in a.iter() {
            sum_a += *v;
            set_a.insert(*v);
        }
        let mut sum_b: i32 = 0;
        let mut set_b: HashSet<i32> = HashSet::new();
        for v in b.iter() {
            sum_b += *v;
            set_b.insert(*v);
        }
        let delta: i32 = (sum_a - sum_b) / 2;
        let mut result: Vec<i32> = Vec::with_capacity(2);
        for v in set_a.iter() {
            let t: i32 = *v - delta;
            if set_b.contains(&t) {
                result.push(*v);
                result.push(t);
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() == n as usize {
                    b.push(t);
                } else {
                    a.push(t);
                }
            }
        }
    }

    if 0 == n || 0 == ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::fair_candy_swap(a, b);
    println!("Fair candy swap: ({}, {})", result[0], result[1]);
}
