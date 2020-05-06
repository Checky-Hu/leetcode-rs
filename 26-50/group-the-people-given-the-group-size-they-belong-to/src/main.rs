use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for (i, v) in group_sizes.iter().enumerate() {
            match map.get_mut(v) {
                Some(x) => x.push(i as i32),
                None => {
                    map.insert(*v, vec![i as i32]);
                }
            }
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        for (k, v) in map.iter() {
            let mut tmp: Vec<i32> = Vec::with_capacity(*k as usize);
            for i in v.iter() {
                tmp.push(*i);
                if tmp.len() == *k as usize {
                    result.push(tmp);
                    tmp = Vec::with_capacity(*k as usize);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut group_sizes: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                group_sizes.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::group_the_people(group_sizes);
    for group in result.iter() {
        for people in group.iter() {
            print!("{} ", *people);
        }
        println!();
    }
}
