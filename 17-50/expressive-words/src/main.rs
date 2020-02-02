use std::env;

struct Solution {}

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut flags: Vec<(u8, i32)> = Vec::new();
        let mut pre_data: (u8, i32) = (0u8, 0i32);
        for u in s.as_bytes() {
            if pre_data.0 == *u {
                pre_data.1 += 1;
            } else {
                if pre_data.0 != 0u8 {
                    flags.push(pre_data);
                }
                pre_data.0 = *u;
                pre_data.1 = 1;
            }
        }
        if pre_data.0 != 0u8 {
            flags.push(pre_data);
        }
        let len: usize = flags.len();
        let mut result: i32 = 0;
        for word in words {
            pre_data.0 = 0u8;
            pre_data.1 = 0i32;
            let mut is_expressive: bool = true;
            let mut i: usize = 0;
            for u in word.as_bytes() {
                if pre_data.0 == *u {
                    pre_data.1 += 1;
                } else {
                    if pre_data.0 != 0u8 {
                        if i >= len
                            || pre_data.0 != flags[i].0
                            || flags[i].1 < pre_data.1
                            || (flags[i].1 != pre_data.1 && flags[i].1 < 3)
                        {
                            pre_data.0 = 0u8;
                            is_expressive = false;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    pre_data.0 = *u;
                    pre_data.1 = 1;
                }
            }
            if pre_data.0 != 0u8
                && (i != len - 1
                    || pre_data.0 != flags[i].0
                    || flags[i].1 < pre_data.1
                    || (flags[i].1 != pre_data.1 && flags[i].1 < 3))
            {
                is_expressive = false;
            }
            if is_expressive {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: String = arg;
                words.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Expressive words count: {}",
        Solution::expressive_words(s, words)
    );
}
