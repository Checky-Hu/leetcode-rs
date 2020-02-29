use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in a.iter() {
            match map.get_mut(v) {
                Some(x) => *x += 1,
                None => {
                    set.insert(*v);
                    map.insert(*v, 1);
                }
            }
        }
        loop {
            let mut change: bool = false;
            let mut next: HashSet<i32> = HashSet::new();
            for v in set.iter() {
                let count: i32 = match map.get(v) {
                    Some(x) => *x,
                    None => 0,
                };
                if count == 0 {
                    continue;
                }
                let use_half: bool = (*v & 1 == 0) && map.contains_key(&(*v / 2));
                let use_double: bool = map.contains_key(&(*v * 2));
                if use_half && use_double {
                    next.insert(*v);
                    continue;
                } else if !use_half && !use_double {
                    return false;
                }
                let target: i32 = if use_half { *v / 2 } else { *v * 2 };
                let mut should_remove: bool = false;
                match map.get_mut(&target) {
                    Some(y) => {
                        if *y >= count {
                            change = true;
                            *y -= count;
                            if *y == 0 {
                                should_remove = true;
                            }
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
                map.remove(v);
                if should_remove {
                    map.remove(&target);
                }
            }
            if !change {
                break;
            }
            set = next;
        }
        for (k, v) in map.iter() {
            if *v != 0 && !(*k == 0 && *v & 1 == 0) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if ret & 1 == 1 {
        println!("Require at least even parameters.");
        return;
    }

    println!("Reorder doubled: {}", Solution::can_reorder_doubled(a));
}
