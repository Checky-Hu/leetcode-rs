use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_prime(n: i32) -> bool {
        if n < 2 || n & 1 == 0 {
            return n == 2;
        }
        for i in 3..(n / 2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    pub fn prime_palindrome(n: i32) -> i32 {
        if n >= 8 && n <= 11 {
            return 11;
        }
        for i in 1i32..=100_000i32 {
            if i == 123 {
                println!("{}", i.reverse_bits());
            }
            let mut tmp_s: String = i.to_string();
            let mut rev_s: Vec<u8> = tmp_s.clone().into_bytes();
            rev_s.reverse();
            rev_s.remove(0);
            tmp_s.push_str(String::from_utf8(rev_s).unwrap_or_default().as_str());
            let tmp_i: i32 = i32::from_str(&tmp_s).unwrap_or(0);
            if tmp_i >= n && Solution::is_prime(tmp_i) {
                return tmp_i;
            }
        }
        1
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Prime palindrome: {}", Solution::prime_palindrome(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
