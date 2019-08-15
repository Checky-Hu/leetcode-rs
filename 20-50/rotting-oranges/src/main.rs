use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
        let column: usize = grid[0].len();

        let mut result: i32 = 0;
        let mut fresh: i32 = 0;
        for v in &grid {
            for n in v {
                if *n == 1 {
                    fresh += 1;
                }
            }
        }
        let mut matrix: Vec<Vec<i32>> = grid.clone();
        while fresh > 0 {
            let pre_fresh: i32 = fresh;
            let mut i: usize = 0;
            while i < row {
                let mut j: usize = 0;
                while j < column {
                    if matrix[i][j] == result + 2 {
                        if i > 0 && matrix[i - 1][j] == 1 {
                            fresh -= 1;
                            matrix[i - 1][j] = result + 3;
                        }
                        if i + 1 < row && matrix[i + 1][j] == 1 {
                            fresh -= 1;
                            matrix[i + 1][j] = result + 3;
                        }
                        if j > 0 && matrix[i][j - 1] == 1 {
                            fresh -= 1;
                            matrix[i][j - 1] = result + 3;
                        }
                        if j + 1 < column && matrix[i][j + 1] == 1 {
                            fresh -= 1;
                            matrix[i][j + 1] = result + 3;
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
            if pre_fresh == fresh {
                return -1
            } else {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
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
                if ret % columns == 0 {
                    grid.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
        return;
    }

    println!("Found: {}", Solution::oranges_rotting(grid));
}
