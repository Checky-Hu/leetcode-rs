extern crate quicksort;

use quicksort::qstuple;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_replace_string(
        s: String,
        indexes: Vec<i32>,
        sources: Vec<String>,
        targets: Vec<String>,
    ) -> String {
        let mut tuples: Vec<(i32, i32)> = Vec::new();
        for (i, p) in indexes.iter().enumerate() {
            tuples.push((*p, i as i32));
        }
        let len: usize = tuples.len();
        qstuple::quick_sort(&mut tuples, 0, len - 1);
        let s_bytes: &[u8] = s.as_bytes();
        let s_len: usize = s_bytes.len();
        let mut cur_pos: usize = 0;
        let mut result: String = String::new();
        for v in tuples.iter() {
            let t_bytes: &[u8] = sources[v.1 as usize].as_bytes();
            let t_len: usize = t_bytes.len();
            let mut delta: usize = 0;
            while v.0 as usize + delta < s_len && delta < t_len {
                if s_bytes[v.0 as usize + delta] != t_bytes[delta] {
                    break;
                } else {
                    delta += 1;
                }
            }
            if delta == t_len {
                for j in cur_pos..(v.0 as usize) {
                    result.push(s_bytes[j] as char);
                }
                result.push_str(&targets[v.1 as usize]);
                cur_pos = v.0 as usize + t_len;
            }
        }
        for j in cur_pos..s_len {
            result.push(s_bytes[j] as char);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut n: i32 = 0;
    let mut indexes: Vec<i32> = Vec::new();
    let mut sources: Vec<String> = Vec::new();
    let mut targets: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if indexes.len() != n as usize {
                    let t: i32 = i32::from_str(&arg).expect("Error parse.");
                    indexes.push(t);
                } else if sources.len() != n as usize {
                    let t: String = arg;
                    sources.push(t);
                } else {
                    let t: String = arg;
                    targets.push(t);
                }
            }
        }
    }

    if 0 == n || 3 * n != ret {
        println!("Require at least (2 + 3 * arg2) parameters.");
        return;
    }

    println!(
        "Replace string: {}",
        Solution::find_replace_string(s, indexes, sources, targets)
    );
}
