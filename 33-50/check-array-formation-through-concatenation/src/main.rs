use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut match_piece: bool = false;
        // (x, y)
        let mut position: (usize, usize) = (0, 0);
        for v in arr.iter() {
            if match_piece {
                if pieces[position.0][position.1] == *v {
                    position.1 += 1;
                    if position.1 == pieces[position.0].len() {
                        match_piece = false;
                    }
                } else {
                    return false;
                }
            } else {
                let mut find_match: bool = false;
                for (i, piece) in pieces.iter().enumerate() {
                    if piece[0] == *v {
                        find_match = true;
                        if piece.len() > 1 {
                            match_piece = true;
                            position.0 = i;
                            position.1 = 1;
                        }
                        break;
                    }
                }
                if !find_match {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut piece: usize = 0;
    let mut count: usize = 0;
    let mut sum: usize = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    let mut pieces: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => piece = usize::from_str(&arg).expect("Error parse."),
            _ => {
                if pieces.len() < piece {
                    if count == 0 {
                        count = usize::from_str(&arg).expect("Error parse.");
                    } else {
                        let num: i32 = i32::from_str(&arg).expect("Error parse.");
                        tmp.push(num);
                        if tmp.len() == count {
                            sum += count;
                            count = 0;
                            pieces.push(tmp);
                            tmp = Vec::new();
                        }
                    }
                } else {
                    let num: i32 = i32::from_str(&arg).expect("Error parse.");
                    arr.push(num);
                }
            }
        }
    }

    if 0 == piece || sum != arr.len() {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Can form array: {}", Solution::can_form_array(arr, pieces));
}
