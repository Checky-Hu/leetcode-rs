use std::env;

struct Solution {}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut answers: Vec<String> = products;
        answers.sort();
        let mut result: Vec<Vec<String>> = Vec::new();
        let bytes: &[u8] = search_word.as_bytes();
        let len: usize = bytes.len();
        for i in 1..=len {
            let mut current: Vec<String> = Vec::with_capacity(3);
            for answer in answers.iter() {
                if answer.len() < i {
                    continue;
                }
                let mut is_match: bool = true;
                let tmp_bytes: &[u8] = answer.as_bytes();
                for j in 0..i {
                    if tmp_bytes[j] != bytes[j] {
                        is_match = false;
                        break;
                    }
                }
                if is_match {
                    current.push(answer.clone());
                    if current.len() == 3 {
                        break;
                    }
                }
            }
            result.push(current);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut search_word: String = String::new();
    let mut products: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => search_word = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                products.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<Vec<String>> = Solution::suggested_products(products, search_word);
    for r in result.iter() {
        for s in r.iter() {
            print!("{} ", *s);
        }
        println!();
    }
}
