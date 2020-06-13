use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let mut map: HashMap<String, i32> = HashMap::new();
        let len: usize = s.len();
        for i in 0..len {
            if let Some(x) = s.get(i..(i + min_size as usize)) {
                let t: String = x.to_string();
                match map.get_mut(&t) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(t, 1);
                    }
                }
            }
        }
        let mut result: i32 = 0;
        for (k, v) in map.iter() {
            let mut set: HashSet<char> = HashSet::new();
            for c in k.chars() {
                set.insert(c);
            }
            if set.len() <= max_letters as usize && *v > result {
                result = *v;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut max_letters: i32 = 0;
    let mut min_size: i32 = 0;
    let mut max_size: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => max_letters = i32::from_str(&arg).expect("Error parse."),
            2 => min_size = i32::from_str(&arg).expect("Error parse."),
            3 => max_size = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                println!(
                    "Maximum numbers of occurrences substring: {}",
                    Solution::max_freq(s, max_letters, min_size, max_size)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 4 parameters.");
    }
}
