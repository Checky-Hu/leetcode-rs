use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn compute(s_bytes: &[u8], l: usize, r: usize, mut map: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        let tmp_s: String = String::from_utf8(s_bytes[l..=r].to_vec()).unwrap();
        match map.get(&tmp_s) {
            Some(x) => x.to_vec(),
            None => {
                let mut tmp_v: Vec<i32> = Vec::new();
                for i in l..=r {
                    match s_bytes[i] {
                        42 | 43 | 45 => {
                            let one: Vec<i32> = Solution::compute(s_bytes, l, i - 1, &mut map);
                            let two: Vec<i32> = Solution::compute(s_bytes, i + 1, r, &mut map);
                            for n1 in &one {
                                for n2 in &two {
                                    match s_bytes[i] {
                                        43 => tmp_v.push(*n1 + *n2),
                                        45 => tmp_v.push(*n1 - *n2),
                                        _ => tmp_v.push((*n1) * (*n2)),
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
                if tmp_v.is_empty() {
                    tmp_v.push(i32::from_str(&tmp_s).expect("Error parse."));
                }
                map.insert(tmp_s, tmp_v.clone());
                tmp_v
            }
        }
    }

    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut map: HashMap<String, Vec<i32>> = HashMap::new();
        Solution::compute(input.as_bytes(), 0, input.len() - 1, &mut map)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if index != 0 {
            ret += 1;
            let input: String = arg;
            let result: Vec<i32> = Solution::diff_ways_to_compute(input);
            for i in result {
                print!("{} ", i);
            }
            print!("\n");
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
