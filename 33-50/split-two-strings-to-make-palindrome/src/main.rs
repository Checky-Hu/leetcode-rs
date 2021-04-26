use std::env;

struct Solution {}

impl Solution {
    fn is_palindrome(vec: &[u8], left: usize, right: usize) -> bool {
        let len: usize = right - left + 1;
        for i in 0..(len >> 1) {
            if vec[left + i] != vec[right - i] {
                return false;
            }
        }
        true
    }

    fn check_palindrome_formation_in_order(a: &[u8], b: &[u8], len: usize) -> bool {
        let mut left: usize = 0;
        let mut right: usize = len - 1;
        while left < right && a[left] == b[right] {
            left += 1;
            right -= 1;
        }
        if left >= right {
            true
        } else {
            Solution::is_palindrome(a, left, right) || Solution::is_palindrome(b, left, right)
        }
    }

    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let a_bytes: &[u8] = a.as_bytes();
        let len: usize = a_bytes.len();
        if len < 2 {
            return true;
        }
        let b_bytes: &[u8] = b.as_bytes();
        Solution::check_palindrome_formation_in_order(a_bytes, b_bytes, len)
            || Solution::check_palindrome_formation_in_order(b_bytes, a_bytes, len)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = arg,
            _ => {
                ret += 1;
                let b: String = arg;
                println!(
                    "Check palindrome formation: {}",
                    Solution::check_palindrome_formation(a, b)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
