use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let len: usize = matrix.len();
        let mut s: i32 = matrix[0][0];
        let mut e: i32 = matrix[len - 1][len - 1];
        while s < e {
            let mid: i32 = s + (e - s) / 2;
            let mut index: i32 = 0;
            let mut row: usize = len - 1;
            let mut column: usize = 0;
            loop {
                if matrix[row][column] <= mid {
                    index += row as i32 + 1;
                    column += 1;
                    if column == len {
                        break;
                    }
                } else {
                    if row == 0 {
                        break;
                    } else {
                        row -= 1;
                    }
                }
            }
            if index < k {
                s = mid + 1;
            } else {
                e = mid;
            }
        }
        s
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut rows: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => rows = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == rows as usize {
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows == 0 || rows * rows != ret {
        println!("Require at least (2 + r * r) parameters.");
        return;
    }

    println!("Kth smallest: {}", Solution::kth_smallest(matrix, k));
}
