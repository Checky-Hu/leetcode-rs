use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let a_len: usize = a.len();
        let mut t: Vec<&[u8]> = Vec::with_capacity(a_len);
        for s in a.iter() {
            t.push(s.as_bytes());
        }
        let s_len: usize = t[0].len();
        let mut visits: Vec<bool> = vec![false; a_len];
        visits[0] = true;
        let mut result: i32 = 0;
        for i in 0..s_len {
            let mut is_sorted: bool = true;
            let mut has_equal: bool = false;
            let mut greater: Vec<usize> = Vec::new();
            for j in 1..t.len() {
                if visits[j] {
                    continue;
                }
                match t[j][i].cmp(&t[j - 1][i]) {
                    Ordering::Less => {
                        is_sorted = false;
                        break;
                    }
                    Ordering::Equal => has_equal = true,
                    Ordering::Greater => greater.push(j),
                }
            }
            if is_sorted {
                if has_equal {
                    for pos in greater {
                        visits[pos] = true;
                    }
                } else {
                    break;
                }
            } else {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            a.push(s);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Min deletion size: {}", Solution::min_deletion_size(a));
}
