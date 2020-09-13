use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_removed_len(pre: &[i32], pre_i: usize, suf: &[i32], suf_i: usize) -> usize {
        if pre[pre_i] <= suf[suf_i] {
            0
        } else {
            let r1: usize = if pre_i == 0 {
                0
            } else {
                Solution::get_removed_len(pre, pre_i - 1, suf, suf_i)
            };
            let r2: usize = if suf_i == 0 {
                0
            } else {
                Solution::get_removed_len(pre, pre_i, suf, suf_i - 1)
            };
            1 + if r1 < r2 { r1 } else { r2 }
        }
    }

    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let len: usize = arr.len();
        let mut prefix: Vec<i32> = Vec::new();
        let mut prefix_len: usize = 0;
        let mut index: usize = 0;
        loop {
            prefix.push(arr[index]);
            prefix_len += 1;
            if index == len - 1 {
                break;
            } else {
                index += 1;
                if arr[index] < arr[index - 1] {
                    break;
                }
            }
        }
        if prefix_len == len {
            return 0;
        }
        let mut suffix: Vec<i32> = Vec::new();
        let mut suffix_len: usize = 0;
        index = len - 1;
        loop {
            suffix.push(arr[index]);
            suffix_len += 1;
            if index == 0 {
                break;
            } else {
                index -= 1;
                if arr[index] > arr[index + 1] {
                    break;
                }
            }
        }
        (len - prefix_len - suffix_len
            + Solution::get_removed_len(&prefix, prefix_len - 1, &suffix, suffix_len - 1))
            as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Shortest subarray to be removed: {}",
        Solution::find_length_of_shortest_subarray(arr)
    );
}
