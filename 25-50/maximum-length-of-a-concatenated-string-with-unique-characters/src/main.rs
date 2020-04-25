use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    fn generate_mask(bytes: &[u8]) -> i32 {
        let mut result: i32 = 0;
        for u in bytes.iter() {
            let t: i32 = 1 << (u - 97);
            if result & t == 0 {
                result += t;
            } else {
                return -1;
            }
        }
        result
    }

    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for s in arr.iter() {
            let len: i32 = s.len() as i32;
            let mask: i32 = Solution::generate_mask(s.as_bytes());
            if mask < 0 {
                continue;
            }
            let mut next: Vec<(i32, i32)> = Vec::new();
            for (k, v) in map.iter() {
                if *k & mask == 0 {
                    next.push((*k + mask, *v + len));
                }
            }
            map.insert(mask, len);
            for v in next {
                map.insert(v.0, v.1);
            }
        }
        let mut result: i32 = 0;
        for v in map.values() {
            if *v > result {
                result = *v;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                arr.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Maximum lenght: {}", Solution::max_length(arr));
}
