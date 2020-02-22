use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut target: Vec<i32> = vec![0; 26];
        for s in b.iter() {
            let mut cur: Vec<i32> = vec![0; 26];
            for u in s.as_bytes().iter() {
                cur[*u as usize - 97] += 1;
            }
            for i in 0..26 {
                if cur[i] > target[i] {
                    target[i] = cur[i];
                }
            }
        }
        let mut result: Vec<String> = Vec::new();
        for s in a {
            let mut cur: Vec<i32> = vec![0; 26];
            for u in s.as_bytes().iter() {
                cur[*u as usize - 97] += 1;
            }
            let mut is_subset: bool = true;
            for i in 0..26 {
                if cur[i] < target[i] {
                    is_subset = false;
                    break;
                }
            }
            if is_subset {
                result.push(s);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<String> = Vec::new();
    let mut b: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let s: String = arg;
                if a.len() == n as usize {
                    b.push(s);
                } else {
                    a.push(s);
                }
            }
        }
    }

    if 0 == n || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<String> = Solution::word_subsets(a, b);
    for s in result.iter() {
        println!("{}", *s);
    }
}
