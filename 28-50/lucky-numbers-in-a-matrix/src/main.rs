use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row: usize = matrix.len();
        let col: usize = matrix[0].len();
        let mut result: Vec<i32> = Vec::with_capacity(row);
        for i in 0..row {
            let mut min: i32 = i32::max_value();
            let mut pos: usize = col;
            for j in 0..col {
                if matrix[i][j] < min {
                    min = matrix[i][j];
                    pos = j;
                }
            }
            let mut is_lucky: bool = true;
            for iter in matrix.iter() {
                if iter[pos] > min {
                    is_lucky = false;
                    break;
                }
            }
            if is_lucky {
                result.push(min);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut col: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
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
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp_row.is_empty() {
        println!("Require at least (1 + arg1 * n) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::lucky_numbers(matrix);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
