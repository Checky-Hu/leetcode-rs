use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set: HashSet<String> = HashSet::new();
        for path in paths.iter() {
            set.insert(path[0].clone());
        }
        for path in paths.iter() {
            if !set.contains(&path[1]) {
                return path[1].clone();
            }
        }
        String::new()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                path.push(s);
                if path.len() == 2 {
                    paths.push(path);
                    path = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Destination city: {}", Solution::dest_city(paths));
}
