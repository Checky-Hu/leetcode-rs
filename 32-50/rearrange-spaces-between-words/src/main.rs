use std::env;

struct Solution {}

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut count: usize = 0;
        let mut words: Vec<String> = Vec::new();
        let mut word: String = String::new();
        let mut prefix: char = ' ';
        for c in text.chars() {
            if c == ' ' {
                count += 1;
                if prefix != ' ' {
                    words.push(word);
                    word = String::new();
                }
            } else {
                word.push(c);
            }
            prefix = c;
        }
        if !word.is_empty() {
            words.push(word);
        }
        let len: usize = words.len();
        let (interval, remainder) = if len == 1 {
            (0, count)
        } else {
            (count / (len - 1), count % (len - 1))
        };
        let mut result: String = String::new();
        for (i, w) in words.iter().enumerate() {
            result.push_str(w);
            if i == len - 1 {
                for _i in 0..remainder {
                    result.push(' ');
                }
            } else {
                for _i in 0..interval {
                    result.push(' ');
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let text: String = arg;
            println!("Reorder spaces: {}", Solution::reorder_spaces(text));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
