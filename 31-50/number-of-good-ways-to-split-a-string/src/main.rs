use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let mut right: Vec<i32> = vec![0; 26];
        for u in bytes.iter() {
            right[*u as usize - 97] += 1;
        }
        let mut count_r: i32 = 0;
        for v in right.iter() {
            if *v > 0 {
                count_r += 1;
            }
        }
        let mut left: Vec<i32> = vec![0; 26];
        let mut count_l: i32 = 0;
        let mut result: i32 = 0;
        for u in bytes.iter() {
            if left[*u as usize - 97] == 0 {
                count_l += 1;
            }
            left[*u as usize - 97] += 1;
            right[*u as usize - 97] -= 1;
            if right[*u as usize - 97] == 0 {
                count_r -= 1;
            }
            match count_l.cmp(&count_r) {
                Ordering::Less => continue,
                Ordering::Equal => result += 1,
                Ordering::Greater => break,
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Number of good ways to split a string: {}",
                Solution::num_splits(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
