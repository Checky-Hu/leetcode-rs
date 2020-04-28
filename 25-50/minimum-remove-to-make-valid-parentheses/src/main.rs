use std::env;

struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut count: usize = 0;
        let mut total: usize = 0;
        let mut result: String = String::new();
        for c in s.chars() {
            match c {
                '(' => {
                    count += 1;
                    total += 1;
                    result.push(c);
                }
                ')' => {
                    if count > 0 {
                        count -= 1;
                        result.push(c);
                    }
                }
                _ => result.push(c),
            }
        }
        if count > 0 {
            let mut i: usize = 0;
            result.retain(|c| {
                if c == '(' {
                    i += 1;
                    i <= total - count
                } else {
                    true
                }
            });
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
            println!(
                "Valid string with min remove: {}",
                Solution::min_remove_to_make_valid(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
