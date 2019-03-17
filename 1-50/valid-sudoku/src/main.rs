use std::env;

struct Solution {
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut flags: Vec<i32>;
	for i in 0..9 {
	    flags = vec![0; 9];
	    for j in 0..9 {
	        if board[i][j] != '.' {
		    flags[board[i][j] as usize - 49] += 1;
		    if flags[board[i][j] as usize - 49] > 1 {
		        return false
		    }
		}
	    }
	}
	for i in 0..9 {
	    flags = vec![0; 9];
	    for j in 0..9 {
	        if board[j][i] != '.' {
		    flags[board[j][i] as usize - 49] += 1;
		    if flags[board[j][i] as usize - 49] > 1 {
		        return false
		    }
		}
	    }
	}
        let mut x: usize = 0;
	let mut y: usize = 0;
	while x < 9 {
	    while y < 9 {
	        flags = vec![0; 9];
	        for i in 0..3 {
		    for j in 0..3 {
		        if board[y + j][x + i] != '.' {
			    flags[board[y + j][x + i] as usize - 49] += 1;
			    if flags[board[y + j][x + i] as usize - 49] > 1 {
			        return false
			    }
			}
		    }
		}
	        y += 3;
	    }
	    y = 0;
	    x += 3;
	}
	true
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut tmp_l: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
	    let mut count: usize = 0;
	    for c in arg.chars() {
	        count += 1;
		tmp_l.push(c);
		if count % 9 == 0 {
		    board.push(tmp_l);
		    tmp_l = Vec::new();
		}
	    }
	    if count == 81 {
	        ret = index;
	    }
	    break;
	}
    }

    if 0 == ret {
        println!("Require one parameter contains 81 chars.");
	return;
    }

    println!("Valid: {}", Solution::is_valid_sudoku(board));
}
