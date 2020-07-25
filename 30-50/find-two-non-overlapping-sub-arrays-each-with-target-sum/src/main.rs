use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let arr_len: usize = arr.len();
        let mut vecs: Vec<(usize, usize, i32)> = Vec::new();
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut sum: i32 = 0;
        while right < arr_len {
            sum += arr[right];
            match sum.cmp(&target) {
                Ordering::Less => (),
                Ordering::Equal => {
                    vecs.push((left, right, (right - left) as i32 + 1));
                }
                Ordering::Greater => {
                    while sum > target {
                        sum -= arr[left];
                        left += 1;
                    }
                    if sum == target {
                        vecs.push((left, right, (right - left) as i32 + 1));
                    }
                }
            }
            right += 1;
        }
        let vecs_len: usize = vecs.len();
        if vecs_len < 2 {
            return -1;
        }
        let max: i32 = i32::max_value();
        let mut min: i32 = max;
        let mut index: usize = 0;
        let mut prefix: Vec<i32> = Vec::with_capacity(arr_len);
        for i in 0..arr_len {
            if vecs[index].1 <= i {
                if vecs[index].2 < min {
                    min = vecs[index].2;
                }
                if index < vecs_len - 1 {
                    index += 1;
                }
            }
            prefix.push(min);
        }
        min = max;
        index = vecs_len - 1;
        let mut suffix: Vec<i32> = vec![max; arr_len];
        let mut j: usize = arr_len - 1;
        loop {
            if vecs[index].0 > j {
                if vecs[index].2 < min {
                    min = vecs[index].2;
                }
                if index > 0 {
                    index -= 1;
                }
            }
            suffix[j] = min;
            if j > 0 {
                j -= 1;
            } else {
                break;
            }
        }
        let mut result: i32 = -1;
        for i in 0..arr_len {
            if prefix[i] != max && suffix[i] != max {
                let t: i32 = prefix[i] + suffix[i];
                if result == -1 || result > t {
                    result = t;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut target: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => target = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Minimum sum of lengths: {}",
        Solution::min_sum_of_lengths(arr, target)
    );
}
