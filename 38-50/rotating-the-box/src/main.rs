use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn rotate_the_box(boxes: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let row: usize = boxes.len();
        let col: usize = boxes[0].len();
        let mut mut_boxes: Vec<Vec<char>> = boxes;
        for row in mut_boxes.iter_mut() {
            let mut count: usize = 0;
            let mut index: usize = 0;
            let mut start: usize = 0;
            while index < col {
                if row[index] == '#' {
                    count += 1;
                } else if row[index] == '*' {
                    for v in row.iter_mut().take(index - count).skip(start) {
                        *v = '.';
                    }
                    for v in row.iter_mut().take(index).skip(index - count) {
                        *v = '#';
                    }
                    count = 0;
                    start = index + 1;
                }
                index += 1;
            }
            for v in row.iter_mut().take(index - count).skip(start) {
                *v = '.';
            }
            for v in row.iter_mut().take(index).skip(index - count) {
                *v = '#';
            }
        }
        let mut result: Vec<Vec<char>> = vec![vec![' '; row]; col];
        for (i, source) in mut_boxes.iter().enumerate().take(row) {
            for (j, target) in result.iter_mut().enumerate().take(col) {
                target[row - i - 1] = source[j];
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut col: usize = 0;
    let mut boxes: Vec<Vec<char>> = Vec::new();
    let mut tmp: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => col = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: char = arg.chars().next().unwrap_or('.');
                tmp.push(c);
                if tmp.len() == col {
                    boxes.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    let result = Solution::rotate_the_box(boxes);
    for row in result.iter() {
        for c in row.iter() {
            print!("{} ", c);
        }
        println!();
    }
}
