use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(a: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::new();
        let mut cur: HashSet<i32> = HashSet::new();
        for m in a.iter() {
            let mut tmp: HashSet<i32> = HashSet::new();
            tmp.insert(*m);
            for n in cur.iter() {
                let t: i32 = *m | *n;
                if !tmp.contains(&t) {
                    tmp.insert(t);
                }
            }
            cur = tmp;
            for n in cur.iter() {
                if !res.contains(n) {
                    res.insert(*n);
                }
            }
        }
        res.len() as i32
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

    println!("Count results: {}", Solution::subarray_bitwise_o_rs(a));
}
