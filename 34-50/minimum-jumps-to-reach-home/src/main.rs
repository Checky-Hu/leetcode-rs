use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let mut result: i32 = 0;
        // (position, backward)
        let mut next: Vec<(i32, bool)> = Vec::new();
        next.push((0, false));
        let mut set: HashSet<i32> = HashSet::new();
        for v in forbidden {
            set.insert(v);
        }
        while !next.is_empty() {
            let mut tmp: Vec<(i32, bool)> = Vec::new();
            for v in next {
                if v.0 == x {
                    return result;
                }
                let mut t: i32 = v.0 + a;
                if t <= 6000 && !set.contains(&t) {
                    tmp.push((t, false));
                    set.insert(t);
                }
                if !v.1 {
                    t = v.0 - b;
                    if t > 0 && !set.contains(&t) {
                        tmp.push((t, true));
                    }
                }
            }
            next = tmp;
            result += 1;
        }
        -1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut x: i32 = 0;
    let mut forbidden: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            3 => x = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                forbidden.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!(
        "Minimum jumps: {}",
        Solution::minimum_jumps(forbidden, a, b, x)
    );
}
