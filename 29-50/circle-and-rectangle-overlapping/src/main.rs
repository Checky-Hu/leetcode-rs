use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let mut distance: i32 = i32::max_value();
        for y in y1..=y2 {
            for x in x1..=x2 {
                let t: i32 = (x - x_center) * (x - x_center) + (y - y_center) * (y - y_center);
                if t < distance {
                    distance = t;
                }
            }
        }
        distance <= radius * radius
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut radius: i32 = 0;
    let mut x_center: i32 = 0;
    let mut y_center: i32 = 0;
    let mut x1: i32 = 0;
    let mut y1: i32 = 0;
    let mut x2: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => radius = i32::from_str(&arg).expect("Error parse."),
            2 => x_center = i32::from_str(&arg).expect("Error parse."),
            3 => y_center = i32::from_str(&arg).expect("Error parse."),
            4 => x1 = i32::from_str(&arg).expect("Error parse."),
            5 => y1 = i32::from_str(&arg).expect("Error parse."),
            6 => x2 = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let y2: i32 = i32::from_str(&arg).expect("Error parse.");
                println!(
                    "Overlap of circle and rectangle: {}",
                    Solution::check_overlap(radius, x_center, y_center, x1, y1, x2, y2)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 7 parameters.");
    }
}
