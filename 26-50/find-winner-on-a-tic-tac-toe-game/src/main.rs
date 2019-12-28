use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; 3]; 3];
        let mut is_a: bool = true;
        for m in &moves {
            grid[m[0] as usize][m[1] as usize] = if is_a { 1 } else { 2 };
            is_a = !is_a;
        }
        for i in 0..3 {
            if ((grid[i][0] != 0) && grid[i][0] == grid[i][1]) && (grid[i][1] == grid[i][2]) {
                if grid[i][0] == 1 {
                    return "A".to_string();
                } else {
                    return "B".to_string();
                }
            } else if ((grid[0][i] != 0) && grid[0][i] == grid[1][i]) && (grid[1][i] == grid[2][i])
            {
                if grid[0][i] == 1 {
                    return "A".to_string();
                } else {
                    return "B".to_string();
                }
            }
        }
        if grid[1][1] != 0
            && (((grid[0][0] == grid[1][1]) && (grid[1][1] == grid[2][2]))
                || ((grid[0][2] == grid[1][1]) && (grid[1][1] == grid[2][0])))
        {
            if grid[1][1] == 1 {
                "A".to_string()
            } else {
                "B".to_string()
            }
        } else if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut moves: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    moves.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!("Result: {}", Solution::tictactoe(moves));
}
