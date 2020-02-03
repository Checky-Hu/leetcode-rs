use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
        let mut backlist: HashSet<i32> = HashSet::new();
        let mut whitelist: HashSet<i32> = HashSet::new();
        let len: usize = fronts.len();
        for i in 0..len {
            if fronts[i] == backs[i] {
                backlist.insert(fronts[i]);
                whitelist.remove(&fronts[i]);
                continue;
            }
            if !backlist.contains(&fronts[i]) {
                whitelist.insert(fronts[i]);
            }
            if !backlist.contains(&backs[i]) {
                whitelist.insert(backs[i]);
            }
        }
        let mut result: i32 = 0;
        for v in whitelist.iter() {
            if result == 0 || *v < result {
                result = *v;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut fronts: Vec<i32> = Vec::new();
    let mut backs: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if fronts.len() == n as usize {
                    backs.push(t);
                } else {
                    fronts.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!("Minimum good number: {}", Solution::flipgame(fronts, backs));
}
