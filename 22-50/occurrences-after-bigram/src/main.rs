use std::env;

struct Solution {
}

impl Solution {
    pub fn find_occurrences(text: String, first: String, second: String) -> Vec<String> {
        let strs: Vec<&str> = text.split_whitespace().collect();
        let len: usize = strs.len();
        let mut result: Vec<String> = Vec::new();
        let mut i: usize = 0;
        while i < len - 2 {
            if strs[i] == first && strs[i + 1] == second {
                result.push(strs[i + 2].to_string());
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut text: String = String::new();
    let mut first: String = String::new();
    let mut second: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            text = arg;
        } else if 2 == index {
            ret += 1;
            first = arg;
        } else if 3 == index {
            ret += 1;
            second = arg;
            break;
        } else {
        }
    }

    if 3 != ret {
        println!("Require at least three parameters.");
        return;
    }

    let result: Vec<String> = Solution::find_occurrences(text, first, second);
    for s in result {
        println!("{}", s);
    }
}

