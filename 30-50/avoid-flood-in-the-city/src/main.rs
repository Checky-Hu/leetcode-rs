use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let len: usize = rains.len();
        let mut result: Vec<i32> = vec![-1; len];
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut positions: Vec<usize> = Vec::new();
        for (i, v) in rains.iter().enumerate() {
            if *v == 0 {
                positions.push(i);
            } else {
                match map.get_mut(v) {
                    Some(x) => {
                        let mut index: usize = positions.len();
                        for (j, position) in positions.iter().enumerate() {
                            if *position > *x {
                                result[*position] = *v;
                                index = j;
                                break;
                            }
                        }
                        if index < positions.len() {
                            positions.remove(index);
                            *x = i;
                        } else {
                            return Vec::new();
                        }
                    }
                    None => {
                        map.insert(*v, i);
                    }
                }
            }
        }
        for position in positions.iter() {
            result[*position] = 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut rains: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                rains.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<i32> = Solution::avoid_flood(rains);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
