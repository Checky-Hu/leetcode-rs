use std::env;

struct Solution {}

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        let len: usize = s.len();
        if len == 0 {
            return 0;
        }
        let mut is_palindrome: bool = true;
        let bytes: &[u8] = s.as_bytes();
        let mut l: usize = 0;
        let mut r: usize = len - 1;
        while l < r {
            if bytes[l] != bytes[r] {
                is_palindrome = false;
                break;
            } else {
                l += 1;
                r -= 1;
            }
        }
        if is_palindrome {
            1
        } else {
            2
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Actions for remove palindrome sub: {}",
                Solution::remove_palindrome_sub(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
