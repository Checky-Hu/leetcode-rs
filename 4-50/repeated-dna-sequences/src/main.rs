use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut current: String = String::new();
        for c in s.chars() {
            if current.len() == 10 {
                current.remove(0);
                current.push(c);
            } else {
                current.push(c);
            }
            if current.len() == 10 {
                match map.get_mut(&current) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(current.clone(), 1);
                    }
                }
            }
        }
        for (k, v) in map.iter() {
            if *v > 1 {
                result.push(k.clone());
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            let result: Vec<String> = Solution::find_repeated_dna_sequences(s);
            println!("Repeated dns sequences:");
            for r in result.iter() {
                println!("{}", *r);
            }
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
