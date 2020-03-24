use std::env;

struct Solution {}

impl Solution {
    pub fn smallest_subsequence(text: String) -> String {
        let mut result: Vec<u8> = Vec::new();
        let mut visits: Vec<bool> = vec![false; 26];
        let mut counts: Vec<i32> = vec![0; 26];
        for u in text.as_bytes() {
            let t: usize = *u as usize - 97;
            counts[t] += 1;
        }
        for u in text.as_bytes() {
            let t: usize = *u as usize - 97;
            counts[t] -= 1;
            if visits[t] {
                continue;
            }
            while let Some(x) = result.last() {
                let c: usize = *x as usize - 97;
                if *x > *u && counts[c] > 0 {
                    visits[c] = false;
                    result.pop();
                } else {
                    break;
                }
            }
            result.push(*u);
            visits[t] = true;
        }
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let text: String = arg;
            println!(
                "Smallest subsequence: {}",
                Solution::smallest_subsequence(text)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
