use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut flags: Vec<HashSet<i32>> = vec![HashSet::new(); num_courses as usize];
        for v in prerequisites {
            flags[v[1] as usize].insert(v[0]);
        }
        let mut list: Vec<i32> = vec![0; num_courses as usize];
        for hashset in &flags {
            for x in hashset.iter() {
                list[*x as usize] += 1;
            }
        }
        let mut result: Vec<i32> = Vec::new();
        for _i in 0..num_courses {
            let mut j: i32 = 0;
            while j < num_courses {
                if list[j as usize] == 0 {
                    break;
                } else {
                    j += 1;
                }
            }
            if j == num_courses {
                return Vec::new()
            } else {
                result.push(j);
                list[j as usize] = -1;
                for x in flags[j as usize].iter() {
                    list[*x as usize] -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut prerequisites: Vec<Vec<i32>> = Vec::new();
    let mut len: i32 = 0;
    let mut tmp_v: Vec<i32> = Vec::new();
    let mut num_courses: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => num_courses = i32::from_str(&arg).expect("Error parse."),
            2 => len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_v.push(number);
                if ret % 2 == 0 {
                    prerequisites.push(tmp_v);
                    tmp_v = Vec::new();
                }
            },
        }
    }

    if 0 == ret || len == 0 || len * 2 != ret as i32 {
        println!("Require at least (2 * len + 2) parameter.");
        return;
    }

    let result: Vec<i32> = Solution::find_order(num_courses, prerequisites);
    for i in result {
        print!("{} ", i);
    }
    print!("\n");
}
