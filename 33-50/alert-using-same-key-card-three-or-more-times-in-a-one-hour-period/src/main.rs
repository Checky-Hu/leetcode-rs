use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_interval(a: (i32, i32), b: (i32, i32)) -> i32 {
        b.0 * 60 + b.1 - a.0 * 60 - a.1
    }

    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
        for (index, name) in key_name.iter().enumerate() {
            let string: Vec<&str> = key_time[index].split(':').collect();
            let number: (i32, i32) = (
                i32::from_str(string[0]).unwrap_or(0),
                i32::from_str(string[1]).unwrap_or(0),
            );
            match map.get_mut(name) {
                Some(x) => x.push(number),
                None => {
                    let vec = vec![number];
                    map.insert(name.to_string(), vec);
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for (k, v) in map.iter_mut() {
            let len: usize = v.len();
            if len < 3 {
                continue;
            }
            v.sort_by(|a, b| match a.0.cmp(&b.0) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => a.1.cmp(&b.1),
                Ordering::Greater => Ordering::Greater,
            });
            let mut prefix: i32 = 61;
            for i in 1..len {
                let t = Solution::count_interval(v[i - 1], v[i]);
                if t + prefix <= 60 {
                    result.push(k.to_string());
                    break;
                }
                prefix = t;
            }
        }
        result.sort();
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut key_name: Vec<String> = Vec::new();
    let mut key_time: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if key_name.len() == len {
                    key_time.push(arg);
                } else {
                    key_name.push(arg);
                }
            }
        }
    }

    if 0 == len || 2 * len > ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    let result = Solution::alert_names(key_name, key_time);
    for s in result.iter() {
        println!("{}", s);
    }
}
