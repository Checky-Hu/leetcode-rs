use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn prev_perm_optl(a: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = a;
        let len: usize = result.len();
        if len == 1 {
            return result;
        }
        let mut i: usize = len - 2;
        let mut has_perm: bool = false;
        loop {
            if result[i] > result[i + 1] {
                has_perm = true;
                break;
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        if has_perm {
            let mut j: usize = len - 1;
            let mut pos: usize = len;
            while j > i {
                if result[j] < result[i] {
                    if pos != len {
                        if result[j] < result[j + 1] {
                            pos = j + 1;
                            break;
                        } else {
                            pos = j;
                        }
                    } else {
                        pos = j;
                    }
                }
                j -= 1;
            }
            result.swap(i, pos);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::prev_perm_optl(a);
    for n in result.iter() {
        print!("{} ", *n);
    }
    println!();
}
