use std::env;

struct Solution {
}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut flags: Vec<usize> = vec![0; 26];
        let mut i: usize = 0;
        for c in order.chars() {
            flags[(c as u8 - 97) as usize] = i;
            i += 1;
        }
        i = 1;
        while i < words.len() {
            let pre: &[u8] = words[i - 1].as_bytes();
            let pre_len: usize = pre.len();
            let cur: &[u8] = words[i].as_bytes();
            let cur_len: usize = cur.len();
            let mut j: usize = 0;
            while j < pre_len {
                if j == cur_len {
                    return false
                } else {
                    if flags[(pre[j] - 97) as usize] > flags[(cur[j] - 97) as usize] {
                        return false
                    } else if flags[(pre[j] - 97) as usize] < flags[(cur[j] - 97) as usize] {
                        break;
                    } else {
                        j += 1;
                    }
                }
            }
            i += 1;
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut order: String = String::new();
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => order = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                words.push(s);
            },
        }
    }

    if order.len() != 26 || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Sorted: {}", Solution::is_alien_sorted(words, order));
}

