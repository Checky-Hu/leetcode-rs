use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut reorder: Vec<String> = folder;
        reorder.sort();
        let mut set: HashSet<String> = HashSet::new();
        for f in reorder.iter() {
            let mut parent: String = f.clone();
            while let Some(x) = parent.pop() {
                if x == '/' && set.contains(&parent) {
                    break;
                }
            }
            if parent.is_empty() {
                set.insert(f.clone());
            }
        }
        let mut result: Vec<String> = Vec::new();
        for v in set.drain() {
            result.push(v);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut folders: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let folder: String = arg;
                folders.push(folder);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<String> = Solution::remove_subfolders(folders);
    for r in result.iter() {
        println!("{}", *r);
    }
}
