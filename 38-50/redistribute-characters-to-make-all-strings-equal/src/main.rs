use std::env;

struct Solution {}

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let len: usize = words.len();
        if len == 1 {
            return true;
        }
        let mut result: Vec<usize> = vec![0; 26];
        for word in words.iter() {
            for c in word.bytes() {
                result[c as usize - 97] += 1;
            }
        }
        for n in result.iter() {
            if *n % len != 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut words: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                words.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Can make equal: {}", Solution::make_equal(words));
}
