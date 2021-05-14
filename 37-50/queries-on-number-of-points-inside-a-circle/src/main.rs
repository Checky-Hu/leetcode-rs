use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for query in queries.iter() {
            let mut tmp: i32 = 0;
            for point in points.iter() {
                if (point[0] - query[0]) * (point[0] - query[0])
                    + (point[1] - query[1]) * (point[1] - query[1])
                    <= query[2] * query[2]
                {
                    tmp += 1;
                }
            }
            result.push(tmp);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(num);
                if points.len() == len {
                    if tmp.len() == 3 {
                        queries.push(tmp);
                        tmp = Vec::new();
                    }
                } else if tmp.len() == 2 {
                    points.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == len || 2 * len > ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    let result = Solution::count_points(points, queries);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
