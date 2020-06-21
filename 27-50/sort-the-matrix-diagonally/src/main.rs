use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row: usize = mat.len();
        let col: usize = mat[0].len();
        let mut result: Vec<Vec<i32>> = mat;
        for i in 0..row {
            let mut tmp: Vec<i32> = Vec::new();
            let mut j: usize = 0;
            while i + j < row && j < col {
                tmp.push(result[i + j][j]);
                j += 1;
            }
            tmp.sort();
            for k in 0..j {
                result[i + k][k] = tmp[k];
            }
        }
        for i in 1..col {
            let mut tmp: Vec<i32> = Vec::new();
            let mut j: usize = 0;
            while i + j < col && j < row {
                tmp.push(result[j][i + j]);
                j += 1;
            }
            tmp.sort();
            for k in 0..j {
                result[k][i + k] = tmp[k];
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: i32 = 0;
    let mut mat: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = i32::from_str(&arg).expect("Error parse."),
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
        println!("Require at least (1 + n * arg1) parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::diagonal_sort(mat);
    for row in result.iter() {
        for c in row.iter() {
            print!("{} ", *c);
        }
        println!();
    }
}
