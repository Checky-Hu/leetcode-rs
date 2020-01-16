extern crate quicksort;

use quicksort::qsstr;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn find(mail: &str, root: &HashMap<String, String>) -> String {
        if root[mail] == *mail {
            mail.to_string()
        } else {
            Solution::find(&root[mail], root)
        }
    }

    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut root: HashMap<String, String> = HashMap::new();
        let mut owner: HashMap<String, String> = HashMap::new();
        let mut map: HashMap<String, HashSet<String>> = HashMap::new();
        for account in &accounts {
            for mail in account.iter().skip(1) {
                root.insert(mail.to_string(), mail.to_string());
                owner.insert(mail.to_string(), account[0].clone());
            }
        }
        for account in &accounts {
            let p: String = Solution::find(&account[1], &root);
            for mail in account.iter().skip(2) {
                let tmp: String = Solution::find(mail, &root);
                root.insert(tmp, p.clone());
            }
        }
        for account in &accounts {
            for mail in account.iter().skip(1) {
                let tmp: String = Solution::find(mail, &root);
                match map.get_mut(&tmp) {
                    Some(x) => {
                        (*x).insert(mail.to_string());
                    }
                    None => {
                        let mut set: HashSet<String> = HashSet::new();
                        set.insert(mail.to_string());
                        map.insert(tmp, set);
                    }
                }
            }
        }
        for (k, v) in map.iter() {
            let mut tmp: Vec<String> = vec![owner[k].clone()];
            for val in v.iter() {
                tmp.push(val.to_string());
            }
            let len: usize = tmp.len();
            qsstr::quick_sort_by_dict(&mut tmp, 1, len - 1);
            result.push(tmp);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut accounts: Vec<Vec<String>> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if n == 0 {
                    n = i32::from_str(&arg).expect("Error parse.");
                } else {
                    let s: String = arg;
                    tmp.push(s);
                    if tmp.len() == n as usize {
                        accounts.push(tmp);
                        tmp = Vec::new();
                        n = 0;
                    }
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    let result: Vec<Vec<String>> = Solution::accounts_merge(accounts);
    for r in &result {
        for s in r {
            print!("{} ", s);
        }
        println!();
    }
}
