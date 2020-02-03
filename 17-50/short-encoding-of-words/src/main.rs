use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut result: i32 = 0;
        let mut set: HashSet<String> = HashSet::new();
        for word in words {
            let mut suffix: Option<String> = None;
            let cur_bytes: &[u8] = word.as_bytes();
            let cur_l: usize = cur_bytes.len();
            let mut tmp_l: usize = 0;
            for v in set.iter() {
                let mut is_suffix: bool = true;
                let tmp_bytes: &[u8] = v.as_bytes();
                tmp_l = tmp_bytes.len();
                let mut i: usize = 1;
                loop {
                    if tmp_bytes[tmp_l - i] != cur_bytes[cur_l - i] {
                        is_suffix = false;
                        break;
                    }
                    i += 1;
                    if i > cur_l || i > tmp_l {
                        break;
                    }
                }
                if is_suffix {
                    suffix = Some(v.clone());
                    break;
                }
            }
            match suffix {
                Some(x) => {
                    if tmp_l < cur_l {
                        set.remove(&x);
                        set.insert(word);
                        result += (cur_l - tmp_l) as i32;
                    }
                }
                None => {
                    set.insert(word);
                    result += cur_l as i32 + 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: String = arg;
                words.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Minimum lenght encoding: {}",
        Solution::minimum_length_encoding(words)
    );
}
