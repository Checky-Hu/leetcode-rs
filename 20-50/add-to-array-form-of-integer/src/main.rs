use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut d: i32 = 1;
        while d <= k {
            d *= 10;
        }
        d /= 10;
        let mut b: Vec<i32> = Vec::new();
        let mut m: i32 = k;
        while d > 0 {
            let tmp: i32 = m / d;
            b.push(tmp);
            m -= tmp * d;
            d /= 10;
        }

        let a_len: usize = a.len();
        let b_len: usize = b.len();
        let mut a_i: usize = a_len - 1;
        let mut b_i: usize = if b_len == 0 {
            b_len
        } else {
            b_len - 1
        };
        let mut carry: i32 = 0;
        let mut result: Vec<i32> = Vec::new();
        loop {
            let tmp: i32;
            if b_i != b_len {
                tmp = a[a_i] + b[b_i] + carry;
                if b_i == 0 {
                    b_i = b_len;
                } else {
                    b_i -= 1;
                }
            } else {
                tmp = a[a_i] + carry;
            }
            if tmp >= 10 {
                result.insert(0, tmp - 10);
                carry = 1;
            } else {
                result.insert(0, tmp);
                carry = 0;
            }
            if a_i == 0 {
                break;
            } else {
                a_i -= 1;
            }
        }
        while b_i != b_len {
            let tmp: i32 = b[b_i] + carry;
            if tmp >= 10 {
                result.insert(0, tmp - 10);
                carry = 1;
            } else {
                result.insert(0, tmp);
                carry = 0;
            }
            if b_i == 0 {
                b_i = b_len;
            } else {
                b_i -= 1;
            }
        }
        if carry == 1 {
            result.insert(0, 1);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::add_to_array_form(a, k);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}

