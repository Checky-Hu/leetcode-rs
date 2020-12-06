use std::env;

struct Solution {}

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut indexes: Vec<i32> = vec![-1; 26];
        let mut result: i32 = -1;
        for (i, c) in s.chars().enumerate() {
            let pos: usize = c as usize - 97;
            if indexes[pos] < 0 {
                indexes[pos] = i as i32;
            } else {
                let t: i32 = i as i32 - indexes[pos] - 1;
                if t > result {
                    result = t;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                println!(
                    "Max length between equal characters: {}",
                    Solution::max_length_between_equal_characters(arg)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
