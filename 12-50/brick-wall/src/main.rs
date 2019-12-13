use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for bricks in &wall {
            let mut pre: i32 = 0;
            for i in 0..(bricks.len() - 1) {
                pre += bricks[i];
                match map.get_mut(&pre) {
                    Some(x) => *x += 1,
                    None => {
                        map.insert(pre, 1);
                    },
                }
            }
        }
        let len: i32 = wall.len() as i32;
        let mut result: i32 = 0;
        for v in map.values() {
            if *v > result {
                result = *v;
            }
        }
        len - result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut wall: Vec<Vec<i32>> = Vec::new();
    let mut n: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                if n > 0 {
                    let number: i32 = i32::from_str(&arg).expect("Error parse.");
                    tmp_row.push(number);
                    if n as usize == tmp_row.len() {
                        wall.push(tmp_row);
                        tmp_row = Vec::new();
                        n = 0;
                    }
                } else {
                    n = i32::from_str(&arg).expect("Error parse.");
                }
            },
        }
    }

    if 0 == ret || 0 != n {
        println!("Require at least two parameters.");
        return
    }

    println!("Least bricks: {}", Solution::least_bricks(wall));
}
