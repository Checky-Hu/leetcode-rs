use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for a in arr.iter() {
            match map.get_mut(a) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*a, 1);
                }
            }
        }
        let mut vec: Vec<usize> = Vec::with_capacity(map.len());
        for v in map.values() {
            vec.push(*v);
        }
        vec.sort_by(|a, b| b.cmp(a));
        let len: usize = arr.len() >> 1;
        let mut result: i32 = 0;
        let mut counts: usize = 0;
        for v in vec.iter() {
            counts += *v;
            result += 1;
            if counts >= len {
                break;
            }
        }
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

    println!("Minimum set size: {}", Solution::min_set_size(arr));
}
