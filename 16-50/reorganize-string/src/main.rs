extern crate quicksort;

use quicksort::qsi32;
use std::env;

struct Solution {}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut count: Vec<i32> = vec![0; 26];
        for b in s.as_bytes() {
            count[*b as usize - 97] += 100;
        }
        for (i, v) in count.iter_mut().enumerate() {
            *v += i as i32;
        }
        qsi32::quick_sort(&mut count, 0, 25);
        let len: usize = s.len();
        let mut result: Vec<u8> = vec![0; len];
        let mut index: usize = 1;
        for v in count {
            let u: u8 = (v % 100) as u8 + 97;
            let n: usize = v as usize / 100;
            if n > (len + 1) / 2 {
                return "".to_string();
            }
            for _i in 0..n {
                if index >= len {
                    index = 0;
                }
                result[index] = u;
                index += 2;
            }
        }
        String::from_utf8(result).unwrap()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = 1;
            let s: String = arg;
            println!("Reorganized string: {}", Solution::reorganize_string(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
