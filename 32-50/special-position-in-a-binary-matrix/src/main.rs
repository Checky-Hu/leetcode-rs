use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let row: usize = mat.len();
        let col: usize = mat[0].len();
        let mut result: i32 = 0;
        for i in 0..row {
            let mut index: usize = col;
            for j in 0..col {
                if mat[i][j] == 1 {
                    if index == col {
                        index = j;
                    } else {
                        index = col;
                        break;
                    }
                }
            }
            if index != col {
                let mut is_special: bool = true;
                for (j, v) in mat.iter().enumerate() {
                    if v[index] == 1 && j != i {
                        is_special = false;
                        break;
                    }
                }
                if is_special {
                    result += 1;
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
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
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

    println!("Special position: {}", Solution::num_special(mat));
}
