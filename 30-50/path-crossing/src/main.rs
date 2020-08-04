use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut pos: (i32, i32) = (0, 0);
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        set.insert(pos);
        for c in path.chars() {
            match c {
                'E' => pos.0 += 1,
                'S' => pos.1 -= 1,
                'W' => pos.0 -= 1,
                'N' => pos.1 += 1,
                _ => (),
            }
            if set.contains(&pos) {
                return true;
            } else {
                set.insert(pos);
            }
        }
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let path: String = arg;
                println!("Is path crossing: {}", Solution::is_path_crossing(path));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
