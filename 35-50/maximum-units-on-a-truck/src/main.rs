use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut cap: i32 = truck_size;
        let mut vec: Vec<Vec<i32>> = box_types;
        vec.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut result: i32 = 0;
        for v in vec.iter() {
            if v[0] < cap {
                result += v[0] * v[1];
                cap -= v[0];
            } else {
                result += cap * v[1];
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut truck_size: i32 = 0;
    let mut box_types: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => truck_size = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    box_types.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Maximum units: {}",
        Solution::maximum_units(box_types, truck_size)
    );
}
