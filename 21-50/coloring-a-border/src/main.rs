use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, r0: i32, c0: i32, color: i32) -> Vec<Vec<i32>> {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut visits: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut result: Vec<Vec<i32>> = grid;
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push((r0 as usize, c0 as usize));
        visits[r0 as usize][c0 as usize] = true;
        let src_color: i32 = result[r0 as usize][c0 as usize];
        while !queue.is_empty() {
            let mut next: Vec<(usize, usize)> = Vec::new();
            for q in queue {
                visits[q.0][q.1] = true;
                let mut is_border: bool = q.0 == 0 || q.0 == row - 1 || q.1 == 0 || q.1 == col - 1;
                if q.0 > 0 && !visits[q.0 - 1][q.1] {
                    if result[q.0 - 1][q.1] != src_color {
                        is_border = true;
                    } else {
                        visits[q.0 - 1][q.1] = true;
                        next.push((q.0 - 1, q.1));
                    }
                }
                if q.0 + 1 < row && !visits[q.0 + 1][q.1] {
                    if result[q.0 + 1][q.1] != src_color {
                        is_border = true;
                    } else {
                        visits[q.0 + 1][q.1] = true;
                        next.push((q.0 + 1, q.1));
                    }
                }
                if q.1 > 0 && !visits[q.0][q.1 - 1] {
                    if result[q.0][q.1 - 1] != src_color {
                        is_border = true;
                    } else {
                        visits[q.0][q.1 - 1] = true;
                        next.push((q.0, q.1 - 1));
                    }
                }
                if q.1 + 1 < col && !visits[q.0][q.1 + 1] {
                    if result[q.0][q.1 + 1] != src_color {
                        is_border = true;
                    } else {
                        visits[q.0][q.1 + 1] = true;
                        next.push((q.0, q.1 + 1));
                    }
                }
                if is_border {
                    result[q.0][q.1] = color;
                }
            }
            queue = next;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut r0: i32 = 0;
    let mut c0: i32 = 0;
    let mut color: i32 = 0;
    let mut n: i32 = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => r0 = i32::from_str(&arg).expect("Error parse."),
            2 => c0 = i32::from_str(&arg).expect("Error parse."),
            3 => color = i32::from_str(&arg).expect("Error parse."),
            4 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == n as usize {
                    grid.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least 4 parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::color_border(grid, r0, c0, color);
    for r in result.iter() {
        for n in r.iter() {
            print!("{} ", *n);
        }
        println!();
    }
}
