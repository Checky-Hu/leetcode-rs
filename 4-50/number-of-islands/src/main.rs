use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut result: i32 = 0;
        if grid.is_empty() || grid[0].is_empty() {
            return result
        }

        let r: usize = grid.len();
        let c: usize = grid[0].len();
        let mut flags: Vec<Vec<bool>> = vec![vec![false; c]; r];
        for i in 0..r {
            for j in 0..c {
                if flags[i][j] || grid[i][j] == '0' {
                    continue;
                }
                flags[i][j] = true;
                let mut v: Vec<usize> = vec![i * c + j];
                while !v.is_empty() {
                    let tmp: usize = v.pop().unwrap();
                    let (x, y) = (tmp / c, tmp % c);
                    if x > 0 && !flags[x - 1][y] && grid[x -1][y] == '1' {
                        flags[x - 1][y] = true;
                        v.push(tmp - c);
                    }
                    if x + 1 < r && !flags[x + 1][y] && grid[x + 1][y] == '1' {
                        flags[x + 1][y] = true;
                        v.push(tmp + c);
                    }
                    if y > 0 && !flags[x][y - 1] && grid[x][y - 1] == '1' {
                        flags[x][y - 1] = true;
                        v.push(tmp - 1);
                    }
                    if y + 1 < c && !flags[x][y + 1] && grid[x][y + 1] == '1' {
                        flags[x][y + 1] = true;
                        v.push(tmp + 1);
                    }
                }
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if number == 1 {
		            tmp_row.push('1');
                } else {
                    tmp_row.push('0');
                }
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

    println!("Count: {}", Solution::num_islands(grid));
}
