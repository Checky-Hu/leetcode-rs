use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut result: Vec<Vec<char>> = board.clone();
        let r: usize = result.len();
        let c: usize = result[0].len();
        let mut q: Vec<Vec<i32>> = Vec::new();
        q.push(click);
        while !q.is_empty() {
            let t: Vec<i32> = q.remove(0);
            let y: i32 = t[0];
            let x: i32 = t[1];
            if result[y as usize][x as usize] == 'M' {
                result[y as usize][x as usize] = 'X';
            } else {
                let mut count: u8 = 0;
                let mut neighbors: Vec<Vec<i32>> = Vec::new();
                for i in -1..2 {
                    for j in -1..2 {
                        let tmp_y: i32 = y + i;
                        let tmp_x: i32 = x + j;
                        if tmp_y < 0 || tmp_y >= r as i32 || tmp_x < 0 || tmp_x >= c as i32 {
                            continue;
                        }
                        if result[tmp_y as usize][tmp_x as usize] == 'M' {
                            count += 1;
                        } else if count == 0 && result[tmp_y as usize][tmp_x as usize] == 'E' {
                            neighbors.push(vec![tmp_y, tmp_x]);
                        }
                    }
                }
                if count > 0 {
                    result[y as usize][x as usize] = (count + 48) as char;
                } else {
                    for neighbor in neighbors {
                        result[neighbor[0] as usize][neighbor[1] as usize] = 'B';
                        q.push(neighbor);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut click: Vec<i32> = Vec::with_capacity(2);
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => click.push(i32::from_str(&arg).expect("Error parse.")),
            2 => click.push(i32::from_str(&arg).expect("Error parse.")),
            3 => rows = i32::from_str(&arg).expect("Error parse."),
            4 => columns = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: char = arg.chars().next().unwrap();
                tmp_row.push(c);
                if tmp_row.len() == columns as usize {
                    board.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg3 * arg4 + 4) parameters.");
        return;
    }

    let result: Vec<Vec<char>> = Solution::update_board(board, click);
    for v in result {
        for i in v {
            print!("{} ", i);
        }
        print!("\n");
    }
}
