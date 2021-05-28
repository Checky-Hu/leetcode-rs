use std::collections::BinaryHeap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_valid(heights: &[i32], index: usize, bricks: i32, ladders: i32) -> bool {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for i in 1..=index {
            if heights[i] > heights[i - 1] {
                heap.push(heights[i] - heights[i - 1]);
            }
        }
        let mut bricks_mut: i32 = bricks;
        let mut ladders_mut: i32 = ladders;
        while let Some(x) = heap.pop() {
            if ladders_mut > 0 {
                ladders_mut -= 1;
            } else if bricks_mut >= x {
                bricks_mut -= x;
            } else {
                return false;
            }
        }
        true
    }

    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let len: usize = heights.len();
        let mut s: usize = 0;
        let mut e: usize = len - 1;
        let mut result: i32 = 0;
        while s <= e {
            let middle: usize = (e - s) / 2 + s;
            if Solution::is_valid(&heights, middle, bricks, ladders) {
                result = middle as i32;
                s = middle + 1;
            } else {
                e = middle - 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut bricks: i32 = 0;
    let mut ladders: i32 = 0;
    let mut heights: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => bricks = i32::from_str(&arg).expect("Error parse."),
            2 => ladders = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let height: i32 = i32::from_str(&arg).expect("Error parse.");
                heights.push(height);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!(
        "Furthest building: {}",
        Solution::furthest_building(heights, bricks, ladders)
    );
}
