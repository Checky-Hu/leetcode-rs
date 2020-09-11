use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let len: usize = mat.len();
        let mut result: i32 = 0;
        for (i, v) in mat.iter().enumerate() {
            result += v[i];
            result += v[len - 1 - i];
        }
        if len & 1 == 1 {
            let mid: usize = len / 2;
            result -= mat[mid][mid];
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if tmp_row.len() == n as usize {
                    mat.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || n * n != ret {
        println!("Require at least (1 + arg1 * arg1) parameters.");
        return;
    }

    println!("Diagonal sum: {}", Solution::diagonal_sum(mat));
}
