use std::env;

struct Solution {
}

impl Solution {
    pub fn defang_ip_addr(address: String) -> String {
        let tmp_s: String = "[.]".to_string();
        let mut result: String = String::new();
        for c in address.chars() {
            if c == '.' {
                result.push_str(&tmp_s);
            } else {
                result.push(c);
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
            let address: String = arg;
            println!("IP: {}", Solution::defang_ip_addr(address));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

