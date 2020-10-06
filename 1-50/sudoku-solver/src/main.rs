use std::env;

struct Solution {}

impl Solution {
    fn solve_sudoku_loop(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        rows: &mut Vec<i32>,
        cols: &mut Vec<i32>,
        grid: &mut Vec<Vec<i32>>,
    ) -> bool {
        if i == 9 {
            return true;
        }
        let (next_i, next_j) = if j == 8 { (i + 1, 0) } else { (i, j + 1) };
        if board[i][j] != '.' {
            Solution::solve_sudoku_loop(board, next_i, next_j, rows, cols, grid)
        } else {
            for v in 1_u8..=9_u8 {
                let t: i32 = 1 << v as i32;
                if rows[i] & t == 0 && cols[j] & t == 0 && grid[i / 3][j / 3] & t == 0 {
                    board[i][j] = (v + 48) as char;
                    rows[i] |= t;
                    cols[j] |= t;
                    grid[i / 3][j / 3] |= t;
                    if Solution::solve_sudoku_loop(board, next_i, next_j, rows, cols, grid) {
                        return true;
                    } else {
                        board[i][j] = '.';
                        rows[i] ^= t;
                        cols[j] ^= t;
                        grid[i / 3][j / 3] ^= t;
                    }
                }
            }
            false
        }
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows: Vec<i32> = vec![0; 9];
        let mut cols: Vec<i32> = vec![0; 9];
        let mut grid: Vec<Vec<i32>> = vec![vec![0; 3]; 3];
        for (i, row) in board.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c != '.' {
                    let t: i32 = 1 << (*c as i32 - 48);
                    rows[i] |= t;
                    cols[j] |= t;
                    grid[i / 3][j / 3] |= t;
                }
            }
        }
        Solution::solve_sudoku_loop(board, 0, 0, &mut rows, &mut cols, &mut grid);
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut tmp_l: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            for c in arg.chars() {
                ret += 1;
                tmp_l.push(c);
                if ret % 9 == 0 {
                    board.push(tmp_l);
                    tmp_l = Vec::new();
                }
            }
            break;
        }
    }

    if 81 != ret {
        println!("Require 1 parameter contains 81 chars.");
        return;
    }

    Solution::solve_sudoku(&mut board);
    for row in board.iter() {
        for c in row.iter() {
            print!("{} ", *c);
        }
        println!();
    }
}
