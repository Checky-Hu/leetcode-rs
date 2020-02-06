use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(bytes: &[u8], len: usize, s: usize, result: &mut Vec<i32>) -> bool {
        if s >= len {
            result.len() >= 3
        } else {
            for e in s..len {
                let tmp: i32 =
                    i32::from_str(std::str::from_utf8(&bytes[s..=e]).unwrap_or("-1")).unwrap_or(-1);
                if tmp < 0 || (tmp != 0 && bytes[s] == b'0') {
                    break;
                }
                let l: usize = result.len();
                if l >= 2 {
                    match tmp.cmp(&(result[l - 2] + result[l - 1])) {
                        Ordering::Less => continue,
                        Ordering::Equal => (),
                        Ordering::Greater => break,
                    }
                }
                result.push(tmp);
                if Solution::helper(bytes, len, e + 1, result) {
                    return true;
                } else {
                    result.pop();
                }
            }
            false
        }
    }

    pub fn split_into_fibonacci(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        Solution::helper(bytes, len, 0, &mut result);
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            let result: Vec<i32> = Solution::split_into_fibonacci(s);
            for r in &result {
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
