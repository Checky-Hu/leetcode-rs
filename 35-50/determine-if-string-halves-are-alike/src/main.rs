use std::env;

struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let len: usize = s.len() >> 1;
        let mut prefix: i32 = 0;
        let mut suffix: i32 = 0;
        for (i, c) in s.chars().enumerate() {
            let is_vowel: bool = match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            };
            if is_vowel {
                if i < len {
                    prefix += 1;
                } else {
                    suffix += 1;
                }
            }
        }
        prefix == suffix
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Halves are alike: {}", Solution::halves_are_alike(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
