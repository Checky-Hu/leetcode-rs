use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut vec: Vec<(i32, usize)> = Vec::new();
        for (i, row) in mat.iter().enumerate() {
            let mut count: i32 = 0;
            for r in row.iter() {
                if *r == 1 {
                    count += 1;
                } else {
                    break;
                }
            }
            vec.push((count, i));
        }
        vec.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);
        for i in 0..k {
            result.push(vec[i as usize].1 as i32);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut col: i32 = 0;
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == col as usize {
                    mat.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == col || 0 != ret % col {
        println!("Require at least (2 + n * arg2) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::k_weakest_rows(mat, k);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
