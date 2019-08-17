use std::env;

struct Solution {
}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut result: i32 = 0;
        let mut i: usize = 0;
        while i < 8 {
            let mut j: usize = 0;
            while j < 8 {
                if board[i][j] == 'R' {
                    if i > 0 {
                        let mut k: usize = i - 1;
                        loop {
                            match board[k][j] {
                                'p' => {
                                    result += 1;
                                    break;
                                },
                                'B' => break,
                                _ => (),
                            }
                            if k == 0 {
                                break;
                            } else {
                                k -= 1;
                            }
                        }
                    }
                    if i < 7 {
                        let mut k: usize = i + 1;
                        while k < 8 {
                            match board[k][j] {
                                'p' => {
                                    result += 1;
                                    break;
                                },
                                'B' => break,
                                _ => (),
                            }
                            k += 1;
                        }
                    }
                    if j > 0 {
                        let mut k: usize = j - 1;
                        loop {
                            match board[i][k] {
                                'p' => {
                                    result += 1;
                                    break;
                                },
                                'B' => break,
                                _ => (),
                            }
                            if k == 0 {
                                break;
                            } else {
                                k -= 1;
                            }
                        }
                    }
                    if j < 7 {
                        let mut k: usize = j + 1;
                        while k < 8 {
                            match board[i][k] {
                                'p' => {
                                    result += 1;
                                    break;
                                },
                                'B' => break,
                                _ => (),
                            }
                            k += 1;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut board: Vec<Vec<char>> = Vec::new();
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let c: char = arg.chars().next().unwrap();
                tmp_row.push(c);
                if tmp_row.len() == 8 {
                    board.push(tmp_row);
                    tmp_row = Vec::new();
                    if board.len() == 8 {
                        break;
                    }
                }
            },
        }
    }

    if 0 == ret || 64 != ret {
        println!("Require at least 64 parameter.");
        return;
    }

    println!("Captures: {}", Solution::num_rook_captures(board));
}
