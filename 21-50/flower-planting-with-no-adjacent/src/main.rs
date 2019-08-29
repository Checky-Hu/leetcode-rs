use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let mut flags: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
        for v in paths {
            flags[v[0] as usize - 1].push(v[1]);
            flags[v[1] as usize - 1].push(v[0]);
        }
        let mut result: Vec<i32> = vec![0; n as usize];
        let mut i: usize = 0;
        let mut tmp: Vec<bool> = vec![true; 4];
        while i < n as usize {
            for connect in &flags[i] {
                let cur_garden: usize = *connect as usize - 1;
                if 0 != result[cur_garden] {
                    tmp[result[cur_garden] as usize - 1] = false;
                }
            }
            let mut j: usize = 0;
            while j < 4 {
                if tmp[j] {
                    if result[i] == 0 {
                        result[i] = j as i32 + 1;
                    }
                } else {
                    tmp[j] = true;
                }
                j += 1;
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut paths: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    paths.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == n || 0 == ret || 0 != ret & 1 {
        println!("Require at least (2 * n + 1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::garden_no_adj(n, paths);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
