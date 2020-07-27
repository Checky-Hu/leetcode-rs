use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for a in arr.iter() {
            match map.get_mut(a) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*a, 1);
                }
            }
        }
        let mut vec: Vec<(i32, i32)> = Vec::new();
        for (k, v) in map.iter() {
            vec.push((*k, *v));
        }
        vec.sort_by(|a, b| a.1.cmp(&b.1));
        let mut removed: i32 = 0;
        let mut rest: i32 = k;
        for v in vec.iter() {
            if v.1 <= rest {
                rest -= v.1;
                removed += 1;
            } else {
                break;
            }
        }
        vec.len() as i32 - removed
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

    println!(
        "Least number of unique integers after k removals: {}",
        Solution::find_least_num_of_unique_ints(arr, k)
    );
}
