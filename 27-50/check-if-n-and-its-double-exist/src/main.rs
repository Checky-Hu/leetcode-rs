use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for a in arr.iter() {
            if set.contains(a) {
                return true;
            } else {
                set.insert(*a * 2);
                if *a & 1 == 0 {
                    set.insert(*a / 2);
                }
            }
        }
        false
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

    println!("N and it's double exist: {}", Solution::check_if_exist(arr));
}
