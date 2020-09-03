use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        let mut result: Vec<u8> = Vec::new();
        let mut mut_n: i32 = n;
        let mut count: i32 = 0;
        loop {
            result.push((mut_n % 10) as u8 + 48);
            count += 1;
            mut_n /= 10;
            if mut_n == 0 {
                break;
            } else if count == 3 {
                result.push(46);
                count = 0;
            }
        }
        result.reverse();
        String::from_utf8(result).unwrap_or_default()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Thousand separator: {}", Solution::thousand_separator(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
