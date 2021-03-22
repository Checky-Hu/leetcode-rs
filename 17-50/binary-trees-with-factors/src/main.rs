use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let mut vec: Vec<i32> = arr;
        vec.sort();
        let mut map: HashMap<i32, i64> = HashMap::new();
        let mut result: i64 = 0;
        for n in vec.iter() {
            let mut count: i64 = 1_i64;
            for (k, v) in map.iter() {
                let t: i32 = *n / *k;
                if t * (*k) == *n {
                    if let Some(x) = map.get(&t) {
                        count = ((*v * *x) % modulo + count) % modulo;
                    }
                }
            }
            map.insert(*n, count);
            result = (result + count) % modulo;
        }
        result as i32
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
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Binary trees with factors: {}",
        Solution::num_factored_binary_trees(arr)
    );
}
