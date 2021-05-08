use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut map: HashMap<String, String> = HashMap::new();
        for mut pair in knowledge {
            let val = pair.pop().unwrap_or_default();
            let key = pair.pop().unwrap_or_default();
            map.insert(key, val);
        }
        let mut open_bracket: bool = false;
        let mut key: String = String::new();
        let mut result: String = String::new();
        for c in s.chars() {
            if c == '(' {
                open_bracket = true;
            } else if c == ')' {
                match map.get(&key) {
                    Some(x) => result.push_str(x),
                    None => result.push('?'),
                }
                open_bracket = false;
                key = String::new();
            } else if open_bracket {
                key.push(c);
            } else {
                result.push(c);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut s: String = String::new();
    let mut knowledge: Vec<Vec<String>> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                tmp.push(arg);
                if tmp.len() == 2 {
                    knowledge.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!("Evaluate string: {}", Solution::evaluate(s, knowledge));
}
