use std::env;

struct Solution {
}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        let a_len: i32 = a.len() as i32;
        let b_len: i32 = b.len() as i32;
        if a == b {
            -1
        } else {
            if a_len > b_len {
                a_len
            } else {
                b_len
            }
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            a = arg;
        } else if 2 == index {
            ret += 1;
            let b: String = arg;
            println!("Longest uncommon subsequence length: {}", Solution::find_lu_slength(a.clone(), b));
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
