use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
	    return
	}
	let r: usize = board.len();
	let c: usize = board[0].len();
	for i in 0..r {
	    for j in 0..c {
	        if i != 0 && i != r - 1 && j != 0 && j != c - 1 {
		    continue;
		}
		if board[i][j] != 'O' {
		    continue;
		}
		board[i][j] = '$';
                let mut v: Vec<usize> = vec![i * c + j];
		while !v.is_empty() {
		    let tmp: usize = v.pop().unwrap();
		    let (x, y) = (tmp / c, tmp % c);
		    if x > 0 && board[x - 1][y] == 'O' {
		        board[x - 1][y] = '$';
			v.push(tmp - c);
		    }
		    if x + 1 < r && board[x + 1][y] == 'O' {
		        board[x + 1][y] = '$';
			v.push(tmp + c);
		    }
		    if y > 0 && board[x][y - 1] == 'O' {
		        board[x][y - 1] = '$';
			v.push(tmp - 1);
		    }
		    if y + 1 < c && board[x][y + 1] == 'O' {
		        board[x][y + 1] = '$';
			v.push(tmp + 1);
		    }
		}
	    }
	}
	for i in 0..r {
	    for j in 0..c {
	        if board[i][j] == 'O' {
		    board[i][j] = 'X';
		}
	        if board[i][j] == '$' {
		    board[i][j] = 'O';
		}
	    }
	}
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    0 => (),
	    1 => rows = i32::from_str(&arg).expect("Error parse."),
	    2 => columns = i32::from_str(&arg).expect("Error parse."),
	    _ => {
                let s: String = arg;
		if s.len() == (rows * columns) as usize {
                    let mut tmp_row: Vec<char> = Vec::new();
		    for c in s.chars() {
	                tmp_row.push(c);
		        if tmp_row.len() == columns as usize {
		            board.push(tmp_row);
		            tmp_row = Vec::new();
		        }
		    }
                    ret += 1;
		}
		break;
	    },
	}
    }

    if 0 == ret {
        println!("Require at least (arg1 * arg2) parameters.");
	return;
    }

    Solution::solve(&mut board);
    for v in board {
        for c in v {
            print!("{} ", c);
	}
        print!("\n");
    }
}
