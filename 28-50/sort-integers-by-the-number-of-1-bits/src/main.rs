use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = arr;
        result.sort_by(|a, b| match a.count_ones().cmp(&b.count_ones()) {
            Ordering::Equal => a.cmp(&b),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::sort_by_bits(arr);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
