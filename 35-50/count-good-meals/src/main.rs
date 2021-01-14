use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let max: i32 = 1 << 21;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result: i32 = 0;
        for d in deliciousness {
            let mut val: i32 = 1;
            while val < d {
                val <<= 1;
            }
            while val <= max {
                if let Some(x) = map.get(&(val - d)) {
                    result = (result + *x) % modulo;
                }
                val <<= 1;
            }
            match map.get_mut(&d) {
                Some(x) => *x += 1,
                None => {
                    map.insert(d, 1);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut deliciousness: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                deliciousness.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Good meals: {}", Solution::count_pairs(deliciousness));
}
