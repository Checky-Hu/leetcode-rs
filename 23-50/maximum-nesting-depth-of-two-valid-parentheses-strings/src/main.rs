use std::env;

struct Solution {}

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let len: usize = seq.len();
        let mut a_len: usize = 0;
        let mut b_len: usize = 0;
        let mut result: Vec<i32> = Vec::with_capacity(len);
        for c in seq.chars() {
            match c {
                '(' => {
                    if a_len > b_len {
                        result.push(1);
                        b_len += 1;
                    } else {
                        result.push(0);
                        a_len += 1;
                    }
                }
                _ => {
                    if a_len > b_len {
                        result.push(0);
                        a_len -= 1;
                    } else {
                        result.push(1);
                        b_len -= 1;
                    }
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
            let seq: String = arg;
            let result: Vec<i32> = Solution::max_depth_after_split(seq);
            for r in result.iter() {
                print!("{} ", *r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
