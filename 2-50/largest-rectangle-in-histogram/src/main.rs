use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let len: usize = heights.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut result: i32 = 0;
        for i in 0..len {
            while let Some(x) = stack.last() {
                if heights[*x] > heights[i] {
                    let t: usize = *x;
                    stack.pop();
                    if let Some(y) = stack.last() {
                        result = std::cmp::max(result, (i - *y - 1) as i32 * heights[t]);
                    } else {
                        result = std::cmp::max(result, i as i32 * heights[t]);
                    }
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        while let Some(x) = stack.last() {
            let t: usize = *x;
            stack.pop();
            if let Some(y) = stack.last() {
                result = std::cmp::max(result, (len - *y - 1) as i32 * heights[t]);
            } else {
                result = std::cmp::max(result, len as i32 * heights[t]);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut heights: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let height: i32 = i32::from_str(&arg).expect("Error parse.");
            heights.push(height);
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Largest rectangle area: {}",
        Solution::largest_rectangle_area(heights)
    );
}
