use std::env;

struct Solution {}

impl Solution {
    pub fn max_rep_optl(text: String) -> i32 {
        let bytes: &[u8] = text.as_bytes();
        let len: usize = bytes.len();
        // (start index, end index)
        let mut status: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 26];
        // (character, start index)
        let mut prefix: (u8, usize) = (0, len);
        for (i, u) in bytes.iter().enumerate() {
            if prefix.0 != *u {
                if prefix.0 != 0 {
                    status[prefix.0 as usize - 97].push((prefix.1, i - 1));
                }
                prefix.0 = *u;
                prefix.1 = i;
            }
        }
        status[prefix.0 as usize - 97].push((prefix.1, len - 1));
        let mut result: usize = 0;
        for vec in status.iter() {
            let mut pre_len: usize = 0;
            for i in 0..vec.len() {
                let cur_len: usize = vec[i].1 - vec[i].0 + 1;
                if i == 0 {
                    if cur_len > result {
                        result = cur_len;
                    }
                } else {
                    let t: usize = if vec[i].0 == vec[i - 1].1 + 2 {
                        pre_len + cur_len + if vec.len() > 2 { 1 } else { 0 }
                    } else if pre_len > cur_len {
                        pre_len + 1
                    } else {
                        cur_len + 1
                    };
                    if t > result {
                        result = t;
                    }
                }
                pre_len = cur_len;
            }
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let text: String = arg;
            println!(
                "Maximum length of repeated character substring after swap: {}",
                Solution::max_rep_optl(text)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
