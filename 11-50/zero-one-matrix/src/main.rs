use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row: usize = matrix.len();
        let column: usize = matrix[0].len();
        let mut result: Vec<Vec<i32>> = vec![vec![-1; column]; row];
        let mut queue: Vec<Vec<usize>> = Vec::new();
        for i in 0..row {
            for j in 0..column {
                if matrix[i][j] == 0 {
                    queue.push(vec![i, j]);
                    result[i][j] = 0;
                }
            }
        }
        while !queue.is_empty() {
            let tmp: Vec<usize> = queue.remove(0);
            let y: usize = tmp[0];
            let x: usize = tmp[1];
            if y > 0 && result[y - 1][x] < 0 {
                result[y - 1][x] = result[y][x] + 1;
                queue.push(vec![y - 1, x]);
            }
            if y + 1 < row && result[y + 1][x] < 0 {
                result[y + 1][x] = result[y][x] + 1;
                queue.push(vec![y + 1, x]);
            }
            if x > 0 && result[y][x - 1] < 0 {
                result[y][x - 1] = result[y][x] + 1;
                queue.push(vec![y, x - 1]);
            }
            if x + 1 < column && result[y][x + 1] < 0 {
                result[y][x + 1] = result[y][x] + 1;
                queue.push(vec![y, x + 1]);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => rows = i32::from_str(&arg).expect("Error parse."),
            2 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if columns as usize == tmp_row.len() {
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameters.");
        return
    }

    let result: Vec<Vec<i32>> = Solution::update_matrix(matrix);
    for v in result {
        for n in v {
            print!("{} ", n);
        }
        print!("\n");
    }
}
