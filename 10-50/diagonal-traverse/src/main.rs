use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        if matrix.is_empty() || matrix[0].is_empty() {
            return result
        }
        let r: usize = matrix.len();
        let c: usize = matrix[0].len();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut order: bool = true;
        while result.len() < r * c {
            if order {
                loop {
                    result.push(matrix[i][j]);
                    if i == 0 || j + 1 == c {
                        break;
                    } else {
                        i -= 1;
                        j += 1;
                    }
                }
                if j + 1 == c {
                    i += 1;
                } else {
                    j += 1;
                }
                order = false;
            } else {
                loop {
                    result.push(matrix[i][j]);
                    if j == 0 || i + 1 == r {
                        break;
                    } else {
                        i += 1;
                        j -= 1;
                    }
                }
                if i + 1 == r {
                    j += 1;
                } else {
                    i += 1;
                }
                order = true;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => r = i32::from_str(&arg).expect("Error parse."),
            2 => c = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if c as usize == tmp_row.len() {
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || r * c != ret {
        println!("Require at least (arg1 * arg2) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::find_diagonal_order(matrix);
    for i in result {
        print!("{} ", i);
    }
    print!("\n");
}
