use std::env;

struct Solution {}

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut record: Vec<i32> = vec![0; 26];
        for u in s.as_bytes() {
            record[*u as usize - 97] += 1;
        }
        let mut result: String = String::new();
        loop {
            for (i, v) in record.iter_mut().enumerate() {
                if *v > 0 {
                    *v -= 1;
                    result.push((i as u8 + 97) as char);
                }
            }
            let mut i: usize = 25;
            loop {
                if record[i] > 0 {
                    record[i] -= 1;
                    result.push((i as u8 + 97) as char);
                }
                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
            if result.len() == s.len() {
                break;
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
            let s: String = arg;
            println!("Increasing decreasing string: {}", Solution::sort_string(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
