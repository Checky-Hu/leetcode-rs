use std::env;

struct Solution {}

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut result: Vec<u8> = s.into_bytes();
        let mut has_bad: bool = false;
        loop {
            let mut tmp: Vec<u8> = Vec::new();
            let len: usize = result.len();
            let mut i: usize = 0;
            while i < len {
                if i + 1 < len
                    && (result[i] + 32 == result[i + 1] || result[i] - 32 == result[i + 1])
                {
                    has_bad = true;
                    i += 2;
                } else {
                    tmp.push(result[i]);
                    i += 1;
                }
            }
            if has_bad {
                result = tmp;
                has_bad = false;
            } else {
                break;
            }
        }
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Make good string: {}", Solution::make_good(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
