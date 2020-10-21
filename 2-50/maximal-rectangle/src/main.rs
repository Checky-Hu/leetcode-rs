use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn maximal_rectangle_in_area(vec: &[i32], len: usize) -> i32 {
        let mut left_max: Vec<i32> = vec![-1; len];
        let mut stack: Vec<usize> = Vec::with_capacity(len);
        for i in 0..len {
            while let Some(x) = stack.last() {
                if vec[*x] >= vec[i] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(x) = stack.last() {
                left_max[i] = *x as i32;
            }
            stack.push(i);
        }
        let mut right_max: Vec<i32> = vec![len as i32; len];
        stack.clear();
        let mut j: usize = len - 1;
        loop {
            while let Some(x) = stack.last() {
                if vec[*x] >= vec[j] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if let Some(x) = stack.last() {
                right_max[j] = *x as i32;
            }
            stack.push(j);
            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }
        let mut result: i32 = 0;
        for i in 0..len {
            result = std::cmp::max(result, ((left_max[i] - right_max[i]).abs() - 1) * vec[i]);
        }
        result
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let row: usize = matrix.len();
        if row == 0 {
            return 0;
        }
        let col: usize = matrix[0].len();
        if col == 0 {
            return 0;
        }
        let mut nums: Vec<Vec<i32>> = vec![vec![0; col]; row];
        let mut i: usize = row - 1;
        loop {
            let mut j: usize = col - 1;
            loop {
                if j + 1 < col && matrix[i][j] > '0' && matrix[i][j + 1] > '0' {
                    nums[i][j] = nums[i][j + 1] + 1;
                } else {
                    nums[i][j] = (matrix[i][j] as u8 - 48) as i32;
                }
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        let mut result: i32 = 0;
        for m in 0..col {
            let mut vec: Vec<i32> = Vec::with_capacity(row);
            for r in nums.iter() {
                vec.push(r[m]);
            }
            result = std::cmp::max(result, Solution::maximal_rectangle_in_area(&vec, row));
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut row: usize = 0;
    let mut col: usize = 0;
    let mut tmp_row: Vec<char> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => row = usize::from_str(&arg).expect("Error parse."),
            2 => col = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c: char = arg.chars().next().unwrap();
                tmp_row.push(c);
                if tmp_row.len() == col {
                    matrix.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || row * col != ret {
        println!("Require at least (arg1 * arg2) parameters.");
        return;
    }

    println!("Maximal rectangle: {}", Solution::maximal_rectangle(matrix));
}
