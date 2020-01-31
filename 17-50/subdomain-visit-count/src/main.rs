use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();
        for s in cpdomains {
            let strs: Vec<&str> = s.split_whitespace().collect();
            let num: i32 = i32::from_str(strs[0]).expect("Error parse.");
            match map.get_mut(strs[1]) {
                Some(x) => *x += num,
                None => {
                    map.insert(strs[1].to_string(), num);
                }
            }
            for (i, c) in strs[1].chars().enumerate() {
                if c == '.' {
                    let tmp: String = strs[1].get((i + 1)..).unwrap().to_string();
                    match map.get_mut(&tmp) {
                        Some(x) => *x += num,
                        None => {
                            map.insert(tmp, num);
                        }
                    }
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for (k, v) in map.iter() {
            let mut tmp: String = v.to_string();
            tmp.push(' ');
            tmp.push_str(k);
            result.push(tmp);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut cpdomains: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                cpdomains.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<String> = Solution::subdomain_visits(cpdomains);
    for r in &result {
        println!("{}", *r);
    }
}
