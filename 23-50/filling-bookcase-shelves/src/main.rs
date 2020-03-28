use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let len: usize = books.len();
        let mut dp: Vec<i32> = vec![i32::max_value(); len + 1];
        dp[0] = 0;
        for i in 1..=len {
            let mut height: i32 = 0;
            let mut width: i32 = 0;
            let mut j: usize = i - 1;
            loop {
                width += books[j][0];
                if width > shelf_width {
                    break;
                }
                if books[j][1] > height {
                    height = books[j][1];
                }
                let t: i32 = dp[j] + height;
                if t < dp[i] {
                    dp[i] = t;
                }
                if j == 0 {
                    break;
                } else {
                    j -= 1;
                }
            }
        }
        dp[len]
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut shelf_width: i32 = 0;
    let mut books: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => shelf_width = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(n);
                if tmp_row.len() == 2 {
                    books.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Minimum height shelves: {}",
        Solution::min_height_shelves(books, shelf_width)
    );
}
