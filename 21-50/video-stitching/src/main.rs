use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, t: i32) -> i32 {
        let mut status: Vec<usize> = vec![0; 101];
        for clip in clips.iter() {
            if status[clip[0] as usize] < clip[1] as usize {
                status[clip[0] as usize] = clip[1] as usize;
            }
        }
        let mut result: i32 = 1;
        let mut index: usize = 0;
        let mut end: usize = status[index];
        let mut reach: bool = false;
        loop {
            if end >= t as usize {
                reach = true;
                break;
            }
            let mut next: usize = 0;
            while index <= end {
                if status[index] > next {
                    next = status[index];
                }
                index += 1;
            }
            if next > end {
                end = next;
                result += 1;
            } else {
                break;
            }
        }
        if reach {
            result
        } else {
            -1
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut t: i32 = 0;
    let mut clips: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => t = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    clips.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!("Video stitching: {}", Solution::video_stitching(clips, t));
}
