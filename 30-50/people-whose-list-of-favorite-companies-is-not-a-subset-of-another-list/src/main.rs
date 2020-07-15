use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut val: i32 = 0;
        let mut vec: Vec<Vec<i32>> = Vec::new();
        for companies in favorite_companies {
            let mut nums: Vec<i32> = Vec::new();
            for company in companies {
                nums.push(match map.get(&company) {
                    Some(x) => *x,
                    None => {
                        map.insert(company, val);
                        val += 1;
                        val - 1
                    }
                });
            }
            nums.sort();
            vec.push(nums);
        }
        let mut result: Vec<i32> = Vec::new();
        let len: usize = vec.len();
        for i in 0..len {
            let mut is_answer: bool = true;
            for j in 0..len {
                if i == j {
                    continue;
                }
                let mut is_subset = true;
                let mut dst: usize = 0;
                for src in 0..vec[i].len() {
                    let mut is_found: bool = false;
                    while dst < vec[j].len() {
                        match vec[i][src].cmp(&vec[j][dst]) {
                            Ordering::Equal => {
                                is_found = true;
                                break;
                            },
                            _ => dst += 1,
                        }
                    }
                    if !is_found {
                        is_subset = false;
                        break;
                    }
                }
                if is_subset {
                    is_answer = false;
                    break;
                }
            }
            if is_answer {
                result.push(i as i32);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut favorite_companies: Vec<Vec<String>> = Vec::new();
    let mut next_companies: Vec<String> = Vec::new();
    let mut next_count: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                if next_count == 0 {
                    next_count = i32::from_str(&arg).expect("Error parse.");
                } else {
                    let company: String = arg;
                    next_companies.push(company);
                    if next_companies.len() == next_count as usize {
                        favorite_companies.push(next_companies);
                        next_companies = Vec::new();
                        next_count = 0;
                    }
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    let result: Vec<i32> = Solution::people_indexes(favorite_companies);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
