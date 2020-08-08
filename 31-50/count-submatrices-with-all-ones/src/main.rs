use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let row: usize = mat.len();
        let col: usize = mat[0].len();
        let mut result: i32 = 0;
        for i in 0..row {
            let mut is_valid: Vec<bool> = vec![true; col];
            for j in i..row {
                for k in 0..col {
                    if mat[j][k] == 0 {
                        is_valid[k] = false;
                    }
                }
                let mut current: i32 = 0;
                for v in is_valid.iter() {
                    if *v {
                        current += 1;
                        result += current;
                    } else {
                        current = 0;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: i32 = 0;
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(n);
                if tmp_row.len() == col as usize {
                    mat.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp_row.is_empty() {
        println!("Require at least (1 + arg1 * n) parameters.");
        return;
    }

    println!(
        "Count submatrices with all ones: {}",
        Solution::num_submat(mat)
    );
}
