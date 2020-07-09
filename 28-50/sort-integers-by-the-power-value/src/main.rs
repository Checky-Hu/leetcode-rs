use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        // <num, steps>
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec: Vec<(i32, i32)> = Vec::new();
        for i in lo..=hi {
            let mut stack: Vec<i32> = Vec::new();
            let mut count: i32;
            let mut next: i32 = i;
            loop {
                match map.get_mut(&next) {
                    Some(x) => {
                        count = *x + 1;
                        break;
                    }
                    None => {
                        if next == 1 {
                            count = 1;
                            break;
                        } else {
                            stack.push(next);
                            if next & 1 == 1 {
                                next = 3 * next + 1;
                            } else {
                                next >>= 1;
                            }
                        }
                    }
                }
            }
            while let Some(x) = stack.pop() {
                map.insert(x, count);
                count += 1;
            }
            vec.push((i, count - 1));
        }
        vec.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.0.cmp(&b.0),
            Ordering::Less => Ordering::Less,
        });
        vec[k as usize - 1].0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut lo: i32 = 0;
    let mut hi: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => lo = i32::from_str(&arg).expect("Error parse."),
            2 => hi = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Number at {}: {}", k, Solution::get_kth(lo, hi, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }
}
