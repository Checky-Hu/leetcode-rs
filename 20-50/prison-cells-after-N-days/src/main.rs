use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn usize_to_vec(size: usize) -> Vec<i32> {
        let mut t: i32 = size as i32;
        let mut vec: Vec<i32> = vec![0; 8];
        let mut i: usize = 7;
        loop {
            vec[i] = t & 1;
            t >>= 1;
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        vec
    }

    fn vec_to_usize(vec: Vec<i32>) -> usize {
        let mut i: usize = 7;
        let mut multi: usize = 1;
        let mut index: usize = 0;
        loop {
            index += multi * vec[i] as usize;
            multi <<= 1;
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        index
    }

    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        let mut index: usize = Solution::vec_to_usize(cells);
        let mut answer: Vec<usize> = vec![256; 256];
        let mut i: i32 = 0;
        while i < n {
            let curr: Vec<i32> = Solution::usize_to_vec(index);
            let mut next: Vec<i32> = vec![0; 8];
            for k in 1..7 {
                next[k] = if curr[k - 1] == curr[k + 1] { 1 } else { 0 };
            }
            answer[index] = Solution::vec_to_usize(next);
            index = answer[index];
            if answer[index] != 256 {
                break;
            } else {
                i += 1;
            }
        }
        let rest: i32 = (n - 1 - i) % 14;
        for _i in 0..rest {
            index = answer[index];
        }
        Solution::usize_to_vec(index)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = -1;
    let mut cells: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let v: i32 = i32::from_str(&arg).expect("Error parse.");
                cells.push(v);
            }
        }
    }

    if -1 == n || 8 != ret {
        println!("Require at least nine parameters.");
        return;
    }

    let result: Vec<i32> = Solution::prison_after_n_days(cells, n);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
