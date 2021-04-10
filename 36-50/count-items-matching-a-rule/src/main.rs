use std::env;

struct Solution {}

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let mut result: i32 = 0;
        let i: usize = if "type" == rule_key {
            0
        } else if "color" == rule_key {
            1
        } else {
            2
        };
        for v in items.iter() {
            if v[i] == rule_value {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut rule_key: String = String::new();
    let mut rule_value: String = String::new();
    let mut items: Vec<Vec<String>> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rule_key = arg,
            2 => rule_value = arg,
            _ => {
                ret += 1;
                tmp.push(arg);
                if tmp.len() == 3 {
                    items.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 3 > ret {
        println!("Require at least 5 parameters.");
        return;
    }

    println!(
        "Items matching a rule: {}",
        Solution::count_matches(items, rule_key, rule_value)
    );
}
