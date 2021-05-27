use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counts: Vec<i32> = vec![0; 26];
        for c in s.chars() {
            counts[c as usize - 97] += 1;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut set: HashSet<i32> = HashSet::new();
        for count in counts {
            if count > 0 {
                set.insert(count);
                match map.get_mut(&count) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(count, 1);
                    }
                }
            }
        }
        let mut result: i32 = 0;
        for (k, v) in map.iter() {
            for _i in 2..=(*v) {
                let mut tmp: i32 = *k;
                while tmp > 0 && set.contains(&tmp) {
                    tmp -= 1;
                }
                if tmp > 0 {
                    set.insert(tmp);
                }
                result += *k - tmp;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Minimum deletions: {}", Solution::min_deletions(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
