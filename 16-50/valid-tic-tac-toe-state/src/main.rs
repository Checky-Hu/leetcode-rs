use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut tmp: Vec<Vec<u8>> = Vec::with_capacity(3);
        for s in board {
            tmp.push(s.into_bytes());
        }
        let mut count_x: i32 = 0;
        let mut count_o: i32 = 0;
        for t in &tmp {
            for u in t {
                match *u {
                    79 => count_o += 1,
                    88 => count_x += 1,
                    _ => (),
                }
            }
        }
        if count_o != count_x && count_o + 1 != count_x {
            return false;
        }
        let mut x_win: i32 = 0;
        let mut o_win: i32 = 0;
        for i in 0..3 {
            if tmp[i][0] != 32 && tmp[i][0] == tmp[i][1] && tmp[i][0] == tmp[i][2] {
                if tmp[i][0] == 79 {
                    o_win += 1;
                } else {
                    x_win += 1;
                }
            }
            if tmp[0][i] != 32 && tmp[0][i] == tmp[1][i] && tmp[0][i] == tmp[2][i] {
                if tmp[0][i] == 79 {
                    o_win += 1;
                } else {
                    x_win += 1;
                }
            }
        }
        if tmp[0][0] != 32 && tmp[0][0] == tmp[1][1] && tmp[0][0] == tmp[2][2] {
            if tmp[0][0] == 79 {
                o_win += 1;
            } else {
                x_win += 1;
            }
        }
        if tmp[0][2] != 32 && tmp[0][2] == tmp[1][1] && tmp[0][2] == tmp[2][0] {
            if tmp[0][2] == 79 {
                o_win += 1;
            } else {
                x_win += 1;
            }
        }
        match x_win.cmp(&1) {
            Ordering::Greater => true,
            Ordering::Equal => {
                if o_win == 1 {
                    false
                } else {
                    count_o + 1 == count_x
                }
            }
            Ordering::Less => {
                if o_win == 1 {
                    count_o == count_x
                } else {
                    true
                }
            }
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut board: Vec<String> = Vec::with_capacity(3);
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                board.push(s);
                if ret == 3 {
                    println!("Valid tic tac toe: {}", Solution::valid_tic_tac_toe(board));
                    break;
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
    }
}
