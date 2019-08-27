use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let (min, mid, max) = if a > b {
            if b > c {
                (c, b, a)
            } else {
                if a > c {
                    (b, c, a)
                } else {
                    (b, a, c)
                }
            }
        } else {
            if a > c {
                (c, a, b)
            } else {
                if b > c {
                    (a, c, b)
                } else {
                    (a, b, c)
                }
            }
        };
        let mut result: Vec<i32> = vec![0, max - min - 2];
        let (dist1, dist2) = (mid - 1 - min, max - 1 - mid);
        if dist1 + dist2 != 0 {
            if dist1 <= 1 || dist2 <= 1 {
                result[0] = 1;
            } else {
                result[0] = 2;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            a = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            b = i32::from_str(&arg).expect("Error parse.");
        } else if 3 == index {
            ret += 1;
            c = i32::from_str(&arg).expect("Error parse.");
            break;
        } else {
            continue;
        }
    }

    if 3 != ret {
        println!("Require at least 3 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::num_moves_stones(a, b, c);
    println!("[{}, {}]", result[0], result[1]);
}
