use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut index: usize = 0;
        let mut word: String = String::new();
        let mut vec: Vec<(String, usize)> = Vec::new();
        for c in text.chars() {
            if c == ' ' {
                vec.push((word, index));
                index += 1;
                word = String::new();
            } else {
                word.push(c.to_ascii_lowercase());
            }
        }
        if !word.is_empty() {
            vec.push((word, index));
        }
        vec.sort_by(|a, b| match a.0.len().cmp(&b.0.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });
        index = 0;
        let mut result: String = String::new();
        for mut v in vec {
            if index == 0 {
                v.0.get_mut(0..1).map(|s| {
                    s.make_ascii_uppercase();
                    &*s
                });
            } else {
                result.push(' ');
            }
            result.push_str(&v.0);
            index += 1;
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
            println!("Rearrange words: {}", Solution::arrange_words(text));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
