use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let len: usize = arr.len();
        let mut status: Vec<(usize, i32)> = Vec::with_capacity(len);
        for (i, n) in arr.iter().enumerate() {
            status.push((i, *n));
        }
        status.sort_by(|a, b| a.1.cmp(&b.1));
        let mut result: Vec<i32> = vec![0; len];
        // (rank, number)
        let mut prefix: (i32, i32) = (0, 0);
        for v in status.iter() {
            if prefix.0 == 0 || prefix.1 != v.1 {
                prefix.0 += 1;
                prefix.1 = v.1;
            }
            result[v.0] = prefix.0;
        }
        result
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::array_rank_transform(arr);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
