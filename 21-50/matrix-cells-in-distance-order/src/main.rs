use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut tmp: Vec<Vec<i32>> = Vec::new();
        let mut flags: Vec<Vec<bool>> = vec![vec![false; c as usize]; r as usize];
        tmp.push(vec![r0, c0]);
        flags[r0 as usize][c0 as usize] = true;
        let mut result: Vec<Vec<i32>> = Vec::new();
        while result.len() != (r * c) as usize {
            let mut cur_list: Vec<Vec<i32>> = Vec::new();
            while !tmp.is_empty() {
                let tmp_v: Vec<i32> = tmp.pop().unwrap();
                if tmp_v[0] > 0 && !flags[(tmp_v[0] - 1) as usize][tmp_v[1] as usize] {
                    cur_list.push(vec![tmp_v[0] - 1, tmp_v[1]]);
                    flags[(tmp_v[0] - 1) as usize][tmp_v[1] as usize] = true;
                }
                if tmp_v[0] < r - 1 && !flags[(tmp_v[0] + 1) as usize][tmp_v[1] as usize] {
                    cur_list.push(vec![tmp_v[0] + 1, tmp_v[1]]);
                    flags[(tmp_v[0] + 1) as usize][tmp_v[1] as usize] = true;
                }
                if tmp_v[1] > 0 && !flags[tmp_v[0] as usize][(tmp_v[1] - 1) as usize] {
                    cur_list.push(vec![tmp_v[0], tmp_v[1] - 1]);
                    flags[tmp_v[0] as usize][(tmp_v[1] - 1) as usize] = true;
                }
                if tmp_v[1] < c - 1 && !flags[tmp_v[0] as usize][(tmp_v[1] + 1) as usize] {
                    cur_list.push(vec![tmp_v[0], tmp_v[1] + 1]);
                    flags[tmp_v[0] as usize][(tmp_v[1] + 1) as usize] = true;
                }
                result.push(tmp_v);
            }
            tmp = cur_list;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut r0: i32 = 0;
    let mut c0: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            r = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            c = i32::from_str(&arg).expect("Error parse.");
        } else if 3 == index {
            ret += 1;
            r0 = i32::from_str(&arg).expect("Error parse.");
        } else if 4 == index {
            ret += 1;
            c0 = i32::from_str(&arg).expect("Error parse.");
            break;
        } else {
            continue;
        }
    }

    if 4 != ret {
        println!("Require at least 4 parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::all_cells_dist_order(r, c, r0, c0);
    for v in result {
        println!("[{}, {}]", v[0], v[1]);
    }
}
