use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let row: usize = grid.len();
        let col: usize = grid[0].len();
        let mut result: Vec<i32> = Vec::with_capacity(col);
        for i in 0..col {
            let mut x: usize = i;
            for v in grid.iter().take(row) {
                if v[x] == 1 {
                    if x + 1 == col || v[x + 1] == -1 {
                        x = col;
                        break;
                    } else {
                        x += 1;
                    }
                } else if x == 0 || v[x - 1] == 1 {
                    x = col;
                    break;
                } else {
                    x -= 1;
                }
            }
            result.push(if x == col { -1 } else { x as i32 });
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut col: usize = 0;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == col {
                    grid.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result = Solution::find_ball(grid);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
