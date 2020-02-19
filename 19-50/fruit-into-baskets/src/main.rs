use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn total_fruit(tree: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut s: usize = 0;
        let mut result: i32 = 0;
        for i in 0..tree.len() {
            match map.get_mut(&tree[i]) {
                Some(x) => *x += 1,
                None => {
                    map.insert(tree[i], 1);
                }
            }
            while map.len() > 2 {
                if let Some(x) = map.get_mut(&tree[s]) {
                    *x -= 1;
                    if *x == 0 {
                        map.remove(&tree[s]);
                    }
                    s += 1;
                }
            }
            let t: i32 = (i + 1 - s) as i32;
            if t > result {
                result = t;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut tree: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            tree.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Total fruit: {}", Solution::total_fruit(tree));
}
