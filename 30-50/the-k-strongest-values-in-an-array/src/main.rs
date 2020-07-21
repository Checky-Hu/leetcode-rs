use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let len: usize = arr.len();
        let mut tmp: Vec<i32> = arr;
        tmp.sort();
        let median: i32 = tmp[(len - 1) / 2];
        tmp.sort_by(|a, b| match (a - median).abs().cmp(&(b - median).abs()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.cmp(&b),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);
        for i in 1..=k {
            result.push(tmp[len - i as usize]);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<i32> = Solution::get_strongest(arr, k);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
