extern crate quicksort;

use quicksort::qsi32;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut t: Vec<i32> = a.clone();
        qsi32::quick_sort(&mut t, 0, len - 1);
        let mut src: HashMap<i32, i32> = HashMap::new();
        let mut dst: HashMap<i32, i32> = HashMap::new();
        for i in 0..len {
            match src.get_mut(&a[i]) {
                Some(x) => *x += 1,
                None => {
                    src.insert(a[i], 1);
                }
            }
            match dst.get_mut(&t[i]) {
                Some(x) => *x += 1,
                None => {
                    dst.insert(t[i], 1);
                }
            }
            let mut found: bool = true;
            for (k, v) in src.iter() {
                if let Some(x) = dst.get(k) {
                    if *x != *v {
                        found = false;
                        break;
                    }
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                return i as i32 + 1;
            }
        }
        len as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Partition point: {}", Solution::partition_disjoint(a));
}
