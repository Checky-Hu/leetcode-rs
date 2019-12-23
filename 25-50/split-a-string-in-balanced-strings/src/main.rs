use std::env;

struct Solution {}

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut result: i32 = 0;
        let mut count: i32 = 0;
        let mut pre_isl: bool = true;
        for c in s.chars() {
            let isl: bool = c == 'L';
            if count == 0 {
                count += 1;
                pre_isl = isl;
            } else {
                if pre_isl == isl {
                    count += 1;
                } else {
                    count -= 1;
                    if count == 0 {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Split to: {}", Solution::balanced_string_split(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
