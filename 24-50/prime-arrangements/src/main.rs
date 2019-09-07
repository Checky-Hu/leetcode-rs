use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let mask: i64 = 1000000007;
        let mut prime_count: i64 = 0;
        let mut other_count: i64 = 0;
        for i in 1..=n {
            if i == 1 {
                other_count += 1;
            } else {
                let mut is_prime: bool = true;
                let flag: i32 = i / 2;
                let mut j: i32 = 2;
                while j <= flag {
                    if i % j == 0 {
                        is_prime = false;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if is_prime {
                    prime_count += 1;
                } else {
                    other_count += 1;
                }
            }
        }
        let mut result: i64 = 1;
        let mut i: i64 = 1;
        while i <= other_count {
            result *= i;
            if result > mask {
                result %= mask;
            }
            i += 1;
        }
        result %= mask;
        i = 1;
        while i <= prime_count {
            result *= i;
            if result > mask {
                result %= mask;
            }
            i += 1;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Arrangements: {}", Solution::num_prime_arrangements(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
