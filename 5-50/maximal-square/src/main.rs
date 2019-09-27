use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0
        }

        let r: usize = matrix.len();
        let c: usize = matrix[0].len();
        let mut result: i32 = 0;
        let mut flags: Vec<Vec<i32>> = vec![vec![0; c]; r];
        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        flags[i][j] = 1;
                    } else {
                        flags[i][j] = if flags[i - 1][j - 1] < flags[i - 1][j] {
                            if flags[i - 1][j - 1] < flags[i][j - 1] {
                                flags[i - 1][j - 1] + 1
                            } else {
                                flags[i][j - 1] + 1
                            }
                        } else {
                            if flags[i - 1][j] < flags[i][j - 1] {
                                flags[i - 1][j] + 1
                            } else {
                                flags[i][j - 1] + 1
                            }
                        }
                    }
                    if flags[i][j] > result {
                        result = flags[i][j];
                    }
                }
            }
        }
        result * result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rows = i32::from_str(&arg).expect("Error parse."),
            2 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                tmp_row.push(arg.chars().next().unwrap());
                if tmp_row.len() == columns as usize {
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
        return;
    }

    println!("Maximal square: {}", Solution::maximal_square(matrix));
}
