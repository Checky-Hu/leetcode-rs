use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let len: usize = board.len();
        let total: usize = len * len;
        let mut visit: Vec<bool> = vec![false; total];
        visit[0] = true;
        let mut queue: Vec<usize> = vec![0];
        let mut result: i32 = 0;
        while !queue.is_empty() {
            result += 1;
            let mut tmp: Vec<usize> = Vec::new();
            for v in queue {
                for i in 1..7 {
                    let mut t: usize = v + i;
                    if visit[t] {
                        continue;
                    }
                    visit[t] = true;
                    let y: usize = len - 1 - t / len;
                    let x: usize = if y & 1 == len & 1 {
                        len - 1 - (t % len)
                    } else {
                        t % len
                    };
                    if board[y][x] != -1 {
                        t = board[y][x] as usize - 1;
                        if visit[t] {
                            continue;
                        }
                    }
                    if t == total - 1 {
                        return result;
                    } else {
                        tmp.push(t);
                    }
                }
            }
            queue = tmp;
        }
        -1
    }
}

fn main() {
    let mut n: i32 = 0;
    let mut marks: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 3 {
                    marks.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == n {
        println!("Require at least one parameter.");
        return;
    }

    let mut board: Vec<Vec<i32>> = vec![vec![-1; n as usize]; n as usize];
    for mark in marks {
        board[mark[0] as usize][mark[1] as usize] = mark[2];
    }
    println!("Steps: {}", Solution::snakes_and_ladders(board));
}
