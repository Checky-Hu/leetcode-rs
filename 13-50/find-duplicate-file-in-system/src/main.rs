use std::collections::HashMap;
use std::env;

struct Solution {
}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for path in paths {
            let mut has_root: bool = false;
            let mut root: String = String::new();
            let mut file: String = String::new();
            let mut temp: String = String::new();
            for c in path.chars() {
                match c {
                    ' ' => {
                        if !has_root {
                            root = temp;
                            temp = String::new();
                            has_root = true;
                        }
                    },
                    '(' => {
                        file = temp;
                        temp = String::new();
                    },
                    ')' => {
                        let cur: String = root.clone() + "/" + &file;
                        match map.get_mut(&temp) {
                            Some(x) => x.push(cur),
                            None => {
                                map.insert(temp, vec![cur; 1]);
                            },
                        }
                        temp = String::new();
                    },
                    _ => {
                        temp.push(c);
                    }
                }
            }
        }
        let mut result: Vec<Vec<String>> = Vec::new();
        for value in map.values() {
            if value.len() > 1 {
                result.push(value.to_vec());
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut paths: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            paths.push(arg);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    let result: Vec<Vec<String>> = Solution::find_duplicate(paths);
    let mut i: i32 = 0;
    for v in result {
        println!("{}:", i);
        for p in v {
            println!("{}", p);
        }
        i += 1;
    }
}
