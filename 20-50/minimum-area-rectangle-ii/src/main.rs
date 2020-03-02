use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let len: usize = points.len();
        let mut result: f64 = 0f64;
        for i in 0..(len - 3) {
            for j in (i + 1)..(len - 2) {
                for k in (j + 1)..(len - 1) {
                    let b1: i32 = (points[i][0] - points[j][0]) * (points[i][0] - points[j][0])
                        + (points[i][1] - points[j][1]) * (points[i][1] - points[j][1]);
                    let b2: i32 = (points[i][0] - points[k][0]) * (points[i][0] - points[k][0])
                        + (points[i][1] - points[k][1]) * (points[i][1] - points[k][1]);
                    let b3: i32 = (points[j][0] - points[k][0]) * (points[j][0] - points[k][0])
                        + (points[j][1] - points[k][1]) * (points[j][1] - points[k][1]);
                    let (p1, p2, target, o) = if b1 + b2 == b3 {
                        (j, k, b3, i)
                    } else if b1 + b3 == b2 {
                        (i, k, b2, j)
                    } else if b2 + b3 == b1 {
                        (i, j, b1, k)
                    } else {
                        (len, len, 0, 0)
                    };
                    if p1 == len {
                        continue;
                    }
                    for l in (k + 1)..len {
                        let t1: i32 = (points[p1][0] - points[l][0])
                            * (points[p1][0] - points[l][0])
                            + (points[p1][1] - points[l][1]) * (points[p1][1] - points[l][1]);
                        let t2: i32 = (points[p2][0] - points[l][0])
                            * (points[p2][0] - points[l][0])
                            + (points[p2][1] - points[l][1]) * (points[p2][1] - points[l][1]);
                        if t1 + t2 == target
                            && ((points[p1][0] == points[o][0] && points[p2][0] == points[l][0])
                                || ((points[p1][1] - points[o][1]) as f64
                                    / (points[p1][0] - points[o][0]) as f64
                                    - (points[p2][1] - points[l][1]) as f64
                                        / (points[p2][0] - points[l][0]) as f64)
                                    .abs()
                                    < std::f64::EPSILON)
                        {
                            let area: f64 = (t1 as f64).sqrt() * (t2 as f64).sqrt();
                            if result == 0f64 || area < result {
                                result = area;
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    points.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Min area rectangle: {}",
        Solution::min_area_free_rect(points)
    );
}
