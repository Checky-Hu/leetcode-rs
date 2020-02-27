use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut x_map: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut y_map: HashMap<i32, HashSet<i32>> = HashMap::new();
        for point in points.iter() {
            match x_map.get_mut(&point[0]) {
                Some(v) => {
                    v.push(point[1]);
                }
                None => {
                    x_map.insert(point[0], vec![point[1]]);
                }
            }
            match y_map.get_mut(&point[1]) {
                Some(s) => {
                    s.insert(point[0]);
                }
                None => {
                    let mut set: HashSet<i32> = HashSet::new();
                    set.insert(point[0]);
                    y_map.insert(point[1], set);
                }
            }
        }
        let mut result: i32 = 0;
        for (k, v) in x_map.iter() {
            let len: usize = v.len();
            if len < 2 {
                continue;
            }
            for i in 0..(len - 1) {
                let s1 = y_map.get(&v[i]).unwrap();
                for j in (i + 1)..len {
                    let s2 = y_map.get(&v[j]).unwrap();
                    for x in s1.iter() {
                        if *x == *k {
                            continue;
                        }
                        if s2.contains(x) {
                            let t: i32 = (v[i] - v[j]).abs() * (*x - *k).abs();
                            if result == 0 || t < result {
                                result = t;
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

    println!("Min area rectangle: {}", Solution::min_area_rect(points));
}
