use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; k as usize];
        let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for log in logs.iter() {
            match map.get_mut(&log[0]) {
                Some(x) => {
                    x.insert(log[1]);
                }
                None => {
                    let mut set: HashSet<i32> = HashSet::new();
                    set.insert(log[1]);
                    map.insert(log[0], set);
                }
            }
        }
        for set in map.values() {
            let len: usize = set.len();
            if len > 0 {
                result[len - 1] += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut logs: Vec<Vec<i32>> = Vec::new();
    let mut log: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                log.push(num);
                if log.len() == 2 {
                    logs.push(log);
                    log = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 3 parameters.");
        return;
    }

    let result = Solution::finding_users_active_minutes(logs, k);
    for r in result.iter() {
        println!("{} ", *r);
    }
}
