use std::env;

struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts: Vec<i32> = vec![0; 26];
        for c in s.chars() {
            counts[c as usize - 97] += 1;
        }
        let mut visits: Vec<bool> = vec![false; 26];
        let mut result: String = String::new();
        for c in s.chars() {
            let i: usize = c as usize - 97;
            counts[i] -= 1;
            if visits[i] {
                continue;
            }
            while let Some(x) = result.pop() {
                let j: usize = x as usize - 97;
                if x > c && counts[j] > 0 {
                    visits[j] = false;
                } else {
                    result.push(x);
                    break;
                }
            }
            result.push(c);
            visits[i] = true;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Remove duplicate letters: {}",
                Solution::remove_duplicate_letters(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
