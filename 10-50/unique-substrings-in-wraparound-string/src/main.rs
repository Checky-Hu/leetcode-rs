use std::env;

struct Solution {
}

impl Solution {
    pub fn find_substring_in_wraparound_string(p: String) -> i32 {
        let mut pre_b: u8 = 0;
        let mut tmp_len: i32 = 1;
        let mut flags: Vec<i32> = vec![0; 26];
        for b in p.as_bytes() {
            if (pre_b == 122 && *b == 97) || *b == pre_b + 1 {
                tmp_len += 1;
            } else {
                tmp_len = 1;
            }
            if tmp_len > flags[*b as usize - 97] {
                flags[*b as usize - 97] = tmp_len;
            }
            pre_b = *b;
        }
        let mut result: i32 = 0;
        for l in flags {
            result += l;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let p: String = arg;
            println!("Substrings: {}", Solution::find_substring_in_wraparound_string(p));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
