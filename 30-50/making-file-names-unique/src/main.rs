use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut result: Vec<String> = Vec::new();
        for name in names {
            if let Some(x) = map.get(&name) {
                let mut index: i32 = *x;
                let mut tmp: String = name.clone();
                tmp.push('(');
                loop {
                    while let Some(y) = tmp.pop() {
                        if y == '(' {
                            break;
                        }
                    }
                    tmp.push('(');
                    tmp.push_str(&index.to_string());
                    tmp.push(')');
                    index += 1;
                    if !map.contains_key(&tmp) {
                        break;
                    }
                }
                map.insert(name, index);
                map.insert(tmp.clone(), 1);
                result.push(tmp);
            } else {
                map.insert(name.clone(), 1);
                result.push(name);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut names: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let name: String = arg;
                names.push(name);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<String> = Solution::get_folder_names(names);
    for r in result.iter() {
        println!("{}", *r);
    }
}
