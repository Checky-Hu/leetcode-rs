use std::collections::HashMap;
use std::env;

struct Solution {
}

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut result: i32 = 0;
        let bytes: &[u8] = input.as_bytes();
        let len: usize = bytes.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 0);
        let mut level: i32 = 0;
        let mut i: usize = 0;
        while i < len {
            let s: usize = i;
            while i < len && bytes[i] as char != '\n' && bytes[i] as char != '\t' {
                i += 1;
            }
            if i >= len || bytes[i] as char == '\n' {
                let mut j: usize = s;
                let mut is_file: bool = false;
                while j < i {
                    if bytes[j] as char == '.' {
                        is_file = true;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if is_file {
                    let t: i32 = map.get(&level).unwrap() + (i - s) as i32;
                    if t > result {
                        result = t;
                    }
                } else {
                    map.insert(level + 1, map.get(&level).unwrap() + (i - s) as i32 + 1);
                }
                level = 0;
            } else {
                level += 1;
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let input: String = arg;
            println!("Longest path length: {}", Solution::length_longest_path(input));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
