use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in barcodes.iter() {
            match map.get_mut(v) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*v, 1);
                }
            }
        }
        let len: usize = barcodes.len();
        let mut vec: Vec<(i32, i32)> = Vec::new();
        for (k, v) in map.iter() {
            vec.push((*k, *v));
        }
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        let mut result: Vec<i32> = vec![0; len];
        let mut i: usize = 0;
        for v in vec.iter() {
            for _j in 0..v.1 {
                result[i] = v.0;
                i += 2;
                if i >= len {
                    i = 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut barcodes: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                barcodes.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::rearrange_barcodes(barcodes);
    for n in result.iter() {
        print!("{} ", *n);
    }
    println!();
}
