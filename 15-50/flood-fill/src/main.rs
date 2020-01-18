use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let r: usize = image.len();
        let c: usize = image[0].len();
        let old_color: i32 = image[sr as usize][sc as usize];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; c]; r];
        let mut result: Vec<Vec<i32>> = image;
        let mut queue: Vec<Vec<usize>> = vec![vec![sr as usize, sc as usize]];
        visited[sr as usize][sc as usize] = true;
        result[sr as usize][sc as usize] = new_color;
        while !queue.is_empty() {
            let mut tmp: Vec<Vec<usize>> = Vec::new();
            for p in queue {
                let (y, x) = (p[0], p[1]);
                if y > 0 && !visited[y - 1][x] && result[y - 1][x] == old_color {
                    result[y - 1][x] = new_color;
                    visited[y - 1][x] = true;
                    tmp.push(vec![y - 1, x]);
                }
                if x + 1 < c && !visited[y][x + 1] && result[y][x + 1] == old_color {
                    result[y][x + 1] = new_color;
                    visited[y][x + 1] = true;
                    tmp.push(vec![y, x + 1]);
                }
                if y + 1 < r && !visited[y + 1][x] && result[y + 1][x] == old_color {
                    result[y + 1][x] = new_color;
                    visited[y + 1][x] = true;
                    tmp.push(vec![y + 1, x]);
                }
                if x > 0 && !visited[y][x - 1] && result[y][x - 1] == old_color {
                    result[y][x - 1] = new_color;
                    visited[y][x - 1] = true;
                    tmp.push(vec![y, x - 1]);
                }
            }
            queue = tmp;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut sr: i32 = 0;
    let mut sc: i32 = 0;
    let mut new_color: i32 = 0;
    let mut image: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => r = i32::from_str(&arg).expect("Error parse."),
            2 => c = i32::from_str(&arg).expect("Error parse."),
            3 => sr = i32::from_str(&arg).expect("Error parse."),
            4 => sc = i32::from_str(&arg).expect("Error parse."),
            5 => new_color = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == c as usize {
                    image.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret || r * c > ret {
        println!("Require at least (5 + arg1 * arg2) parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::flood_fill(image, sr, sc, new_color);
    for r in &result {
        for color in r {
            print!("{} ", color);
        }
        println!();
    }
}
