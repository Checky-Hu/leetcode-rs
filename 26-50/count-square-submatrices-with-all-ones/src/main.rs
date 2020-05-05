use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let row: usize = matrix.len();
        let col: usize = matrix[0].len();
        let mut sum: Vec<Vec<i32>> = vec![vec![0; col + 1]; row + 1];
        for i in 1..=row {
            for j in 1..=col {
                sum[i][j] =
                    matrix[i - 1][j - 1] + sum[i - 1][j] + sum[i][j - 1] - sum[i - 1][j - 1];
            }
        }
        let mut result: i32 = 0;
        for i in 1..=row {
            for j in 1..=col {
                let mut k: usize = 1;
                while k <= i && k <= j {
                    if sum[i][j] + sum[i - k][j - k] - sum[i - k][j] - sum[i][j - k]
                        == (k * k) as i32
                    {
                        result += 1;
                    }
                    k += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => row = i32::from_str(&arg).expect("Error parse."),
            2 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == col as usize {
                    matrix.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == row || 0 == col || row * col != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!(
        "Number of square submatrices: {}",
        Solution::count_squares(matrix)
    );
}
