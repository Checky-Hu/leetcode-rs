use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut n: usize = a.len();
        if n == 1 {
            return result;
        }
        let mut t: Vec<i32> = a;
        loop {
            let mut pos: usize = n;
            for i in 0..n {
                if t[i] == n as i32 {
                    pos = i;
                    break;
                }
            }
            if pos != n - 1 {
                if pos != 0 {
                    result.push(pos as i32 + 1);
                    for i in 0..((pos + 1) / 2) {
                        t.swap(i, pos - i);
                    }
                }
                result.push(n as i32);
                for i in 0..(n / 2) {
                    t.swap(i, n - 1 - i);
                }
            }
            if n == 2 {
                break;
            } else {
                n -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::pancake_sort(a);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
