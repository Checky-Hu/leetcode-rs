use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    fn get_max_unique_split(
        mut set: &mut HashSet<String>,
        bytes: &[u8],
        len: usize,
        index: usize,
    ) -> i32 {
        if index == len {
            return 0;
        }
        let mut result: i32 = -1;
        let mut string: String = String::new();
        for i in index..len {
            string.push(bytes[i] as char);
            if !set.contains(&string) {
                set.insert(string.clone());
                let r: i32 = Solution::get_max_unique_split(&mut set, bytes, len, i + 1);
                if r > -1 && r + 1 > result {
                    result = r + 1;
                }
                set.remove(&string);
            }
        }
        result
    }

    pub fn max_unique_split(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let mut set: HashSet<String> = HashSet::new();
        Solution::get_max_unique_split(&mut set, bytes, bytes.len(), 0)
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Max number to split: {}", Solution::max_unique_split(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
