use std::env;

struct Solution {}

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut herd: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 26];
        for (i, word) in words.iter().enumerate() {
            let bytes: &[u8] = word.as_bytes();
            herd[bytes[0] as usize - 97].push((i, 1));
        }
        let mut result: i32 = 0;
        for u in s.as_bytes() {
            let tmp: Vec<(usize, usize)> = herd[*u as usize - 97].clone();
            herd[*u as usize - 97] = Vec::new();
            for t in tmp {
                let bytes: &[u8] = words[t.0].as_bytes();
                if t.1 == bytes.len() {
                    result += 1;
                } else {
                    herd[bytes[t.1] as usize - 97].push((t.0, t.1 + 1));
                }
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
                let word: String = arg;
                words.push(word);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    println!(
        "Matching subsequences: {}",
        Solution::num_matching_subseq(s, words)
    );
}
