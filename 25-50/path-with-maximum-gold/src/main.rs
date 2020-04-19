use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_max_gold(
        grid: &[Vec<i32>],
        row: usize,
        col: usize,
        y: usize,
        x: usize,
        visits: &mut Vec<Vec<bool>>,
    ) -> i32 {
        visits[y][x] = true;
        let mut max: i32 = 0;
        if y > 0 && !visits[y - 1][x] && grid[y - 1][x] != 0 {
            let t: i32 = Solution::count_max_gold(grid, row, col, y - 1, x, visits);
            if t > max {
                max = t;
            }
        }
        if y + 1 < row && !visits[y + 1][x] && grid[y + 1][x] != 0 {
            let t: i32 = Solution::count_max_gold(grid, row, col, y + 1, x, visits);
            if t > max {
                max = t;
            }
        }
        if x > 0 && !visits[y][x - 1] && grid[y][x - 1] != 0 {
            let t: i32 = Solution::count_max_gold(grid, row, col, y, x - 1, visits);
            if t > max {
                max = t;
            }
        }
        if x + 1 < col && !visits[y][x + 1] && grid[y][x + 1] != 0 {
            let t: i32 = Solution::count_max_gold(grid, row, col, y, x + 1, visits);
            if t > max {
                max = t;
            }
        }
        visits[y][x] = false;
        grid[y][x] + max
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut visits: Vec<Vec<bool>> = vec![vec![false; col]; row];
        let mut result: i32 = 0;
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] != 0 {
                    let t: i32 = Solution::count_max_gold(&grid, row, col, i, j, &mut visits);
                    if t > result {
                        result = t;
                    }
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
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => row = i32::from_str(&arg).expect("Error parse."),
            2 => col = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == col as usize {
                    grid.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret || row * col != ret {
        println!("Require at least (2 + arg1 * arg2) parameters.");
        return;
    }

    println!("Maximum gold: {}", Solution::get_maximum_gold(grid));
}
