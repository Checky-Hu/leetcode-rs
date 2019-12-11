use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    fn get_root(root: &mut Vec<usize>, index: usize) -> usize {
        let mut i: usize = index;
        while i != root[i] {
            root[i] = root[root[i]];
            i = root[i];
        }
        i
    }

    pub fn find_circle_num(m: Vec<Vec<i32>>) -> i32 {
        let len: usize = m.len();
        let mut result: i32 = len as i32;
        let mut root: Vec<usize> = Vec::with_capacity(len);
        for i in 0..len {
            root.push(i);
        }
        for i in 0..len {
            for j in (i + 1)..len {
                if m[i][j] == 1 {
                    let t1: usize = Solution::get_root(&mut root, i);
                    let t2: usize = Solution::get_root(&mut root, j);
                    if t1 != t2 {
                        result -= 1;
                        root[t2] = t1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut m: Vec<Vec<i32>> = Vec::new();
    let mut n: i32 = 0;
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if n as usize == tmp_row.len() {
                    m.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || n * n != ret {
        println!("Require at least (arg1 * arg1) parameters.");
        return
    }

    println!("Circle num: {}", Solution::find_circle_num(m));
}
