use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_root(group: &[i32], index: i32) -> i32 {
        let mut i: i32 = index;
        while group[i as usize] != i {
            i = group[i as usize];
        }
        i
    }

    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let len: usize = connections.len();
        if len < n as usize - 1 {
            return -1;
        }
        let mut group: Vec<i32> = Vec::with_capacity(n as usize);
        for i in 0..n {
            group.push(i);
        }
        for connection in connections.iter() {
            let index1: i32 = Solution::get_root(&group, connection[0]);
            let index2: i32 = Solution::get_root(&group, connection[1]);
            if index1 < index2 {
                group[index2 as usize] = index1;
            } else {
                group[index1 as usize] = index2;
            }
        }
        let mut visits: Vec<bool> = vec![false; n as usize];
        let mut result: i32 = 0;
        for i in 0..n {
            let index: i32 = Solution::get_root(&group, i);
            if !visits[index as usize] {
                visits[index as usize] = true;
                result += 1;
            }
        }
        result - 1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut connections: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 2 {
                    connections.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }

    println!(
        "Make connected: {}",
        Solution::make_connected(n, connections)
    );
}
