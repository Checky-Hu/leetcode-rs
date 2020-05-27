use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut vec: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let list: Vec<char> = vec!['a', 'b', 'c'];
        for _i in 1..n {
            let mut tmp: Vec<String> = Vec::new();
            for s in vec {
                let c: char = *(s.as_bytes().last().unwrap()) as char;
                for l in list.iter() {
                    if c != *l {
                        let mut t: String = s.clone();
                        t.push(*l);
                        tmp.push(t);
                    }
                }
            }
            vec = tmp;
        }
        vec.sort();
        if vec.len() < k as usize {
            String::new()
        } else {
            vec[k as usize - 1].clone()
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Happy string: {}", Solution::get_happy_string(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
