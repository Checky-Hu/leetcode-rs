use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn least_common_multiple(a: i64, b: i64) -> i64 {
        let (mut less, mut greater) = if a > b { (b, a) } else { (a, b) };
        let result: i64 = less * greater;
        while less > 0 {
            let t: i64 = less;
            less = greater % less;
            greater = t;
        }
        result / greater
    }

    fn count_ugly_number(n: i64, a: i64, b: i64, c: i64) -> i64 {
        let ab: i64 = Solution::least_common_multiple(a, b);
        let ac: i64 = Solution::least_common_multiple(a, c);
        let bc: i64 = Solution::least_common_multiple(b, c);
        let abc: i64 = Solution::least_common_multiple(ab, c);
        n / a + n / b + n / c - n / ab - n / ac - n / bc + n / abc
    }

    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let min: i64 = if a > b {
            if b > c {
                c as i64
            } else {
                b as i64
            }
        } else if a > c {
            c as i64
        } else {
            a as i64
        };
        let mut left: i64 = min;
        let mut right: i64 = n as i64 * min;
        while left <= right {
            let mid: i64 = left + (right - left) / 2;
            let tmp: i64 = Solution::count_ugly_number(mid, a as i64, b as i64, c as i64);
            if tmp > n as i64 {
                right = mid - 1;
            } else if tmp < n as i64 {
                left = mid + 1;
            } else if Solution::count_ugly_number(mid - 1, a as i64, b as i64, c as i64) < n as i64
            {
                return mid as i32;
            } else {
                right = mid - 1;
            }
        }
        left as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => a = i32::from_str(&arg).expect("Error parse."),
            3 => b = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("Number: {}", Solution::nth_ugly_number(n, a, b, c));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least four parameters.");
    }
}
