use std::env;

struct Solution {}

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut result: Vec<u8> = Vec::new();
        let mut counts: i32 = 0;
        let mut length: usize = 0;
        for c in number.chars() {
            if c.is_ascii_digit() {
                if counts == 3 {
                    result.push(b'-');
                    length += 1;
                    counts = 0;
                }
                result.push(c as u8);
                length += 1;
                counts += 1;
            }
        }
        if counts == 1 && length >= 3 {
            result.remove(length - 2);
            result.insert(length - 3, b'-');
        }
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: String = arg;
                println!("Reformat number: {}", Solution::reformat_number(number));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
