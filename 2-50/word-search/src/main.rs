use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 || board[0].len() == 0 {
	    return false
	}

	let len: usize = word.len();
	let row: usize = board.len();
	let column: usize = board[0].len();
	if row * column < len {
	    return false
	}

	// current match character index.
	let mut index: usize = 0;
	// current focused position.
	let mut cur_x: usize = 0;
	let mut cur_y: usize = 0;
	// record positions that is available.
	let mut routes_x: Vec<usize> = Vec::new();
	let mut routes_y: Vec<usize> = Vec::new();
	// record direction for each position.
        // 0: up, 1: right, 2: down, 3: left.
	let mut directions: Vec<i32> = Vec::new();
	while index < len {
	    let target: char = word.chars().nth(index).unwrap();
	    if index == 0 {
	        if board[cur_y][cur_x] == target {
		    // try up direction first.
		    directions.push(0);
		    routes_x.push(cur_x);
		    routes_y.push(cur_y);
		    index += 1;
		} else {
		    if cur_x + 1 < column {
		        cur_x += 1;
		    } else {
		        cur_x = 0;
		        if cur_y + 1 < row {
			    cur_y += 1;
			} else {
			    return false
			}
		    }
		}
	    } else {
		let mut cur_dir_available: bool = true;
	        let cur_dir: i32 = directions[index - 1];
		match cur_dir {
		    0 => {
		        if cur_y > 0 {
		            cur_y -= 1;
			} else {
			    cur_dir_available = false;
			}
		    },
		    1 => {
		        if cur_x + 1 < column {
		            cur_x += 1;
			} else {
			    cur_dir_available = false;
			}
		    },
		    2 => {
		        if cur_y + 1 < row {
		            cur_y += 1;
			} else {
			    cur_dir_available = false;
			}
		    },
		    3 => {
		        if cur_x > 0 {
		            cur_x -= 1;
			} else {
			    cur_dir_available = false;
			}
		    },
		    _ => {
		        // all directions fail, pop current position.
		        index -= 1;
			directions.pop();
			if index == 0 {
			    cur_x = routes_x.pop().unwrap();
			    cur_y = routes_y.pop().unwrap();
		            if cur_x + 1 < column {
		                cur_x += 1;
		            } else {
		                cur_x = 0;
		                if cur_y + 1 < row {
			            cur_y += 1;
			        } else {
			            return false
			        }
		            }
			} else {
			    routes_x.pop();
			    routes_y.pop();
			    cur_x = routes_x[index - 1];
			    cur_y = routes_y[index - 1];
			    directions[index - 1] += 1;
			}
			continue;
		    },
		}

		if cur_dir_available {
		    let mut i: usize = 0;
		    while i < routes_x.len() {
		        if routes_x[i] == cur_x && routes_y[i] == cur_y {
		            // repeat preceding position, not available.
		            cur_dir_available = false;
			    break;
			}
			i += 1;
		    }
		}

		if cur_dir_available && board[cur_y][cur_x] == target {
		    directions.push(0);
		    routes_x.push(cur_x);
		    routes_y.push(cur_y);
		    index += 1;
		} else {
		    // current direction fail, try next direction.
		    cur_x = routes_x[index - 1];
		    cur_y = routes_y[index - 1];
		    directions[index - 1] = cur_dir + 1;
		}
	    }
	}
	true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut word: String = String::new();
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut rows: i32 = 0;
    let mut columns: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
	    1 => word = arg,
	    2 => rows = i32::from_str(&arg).expect("Error parse."),
	    3 => columns = i32::from_str(&arg).expect("Error parse."),
	    4 => {
                let mut tmp_row: Vec<char> = Vec::new();
		for c in arg.chars() {
                    ret += 1;
	            tmp_row.push(c);
		    if ret % columns == 0 {
		        board.push(tmp_row);
		        tmp_row = Vec::new();
		    }
		}
		break;
	    },
	    _ => (),
	}
    }

    if 0 == ret || rows * columns != ret {
        println!("Require at least (arg1 * arg2) parameter.");
	return;
    }

    println!("Found: {}", Solution::exist(board, word));
}
