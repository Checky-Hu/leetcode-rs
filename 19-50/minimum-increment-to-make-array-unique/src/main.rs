use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        if len < 2 {
            return 0;
        }
        let mut t: Vec<i32> = a;
        t.sort();
        let mut pre: i32 = t[0];
        let mut result: i32 = 0;
        for i in 1..len {
            if pre >= t[i] {
                pre += 1;
                result += pre - t[i];
            } else {
                pre = t[i];
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Min increment: {}", Solution::min_increment_for_unique(a));
}
