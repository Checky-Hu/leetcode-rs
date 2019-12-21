use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for a in arr {
            match map.get_mut(&a) {
                Some(x) => *x += 1,
                None => {
                    map.insert(a, 1);
                },
            }
        }
        let mut set: HashSet<i32> = HashSet::new();
        for v in map.values() {
            if set.contains(v) {
                return false
            } else {
                set.insert(*v);
            }
        }
        true
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(number);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Unique occurrences: {}", Solution::unique_occurrences(arr));
}

