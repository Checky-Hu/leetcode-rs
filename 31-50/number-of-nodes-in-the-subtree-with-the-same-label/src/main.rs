use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_sub_trees_loop(
        n: usize,
        connection: &[Vec<usize>],
        bytes: &[u8],
        visits: &mut [bool],
        result: &mut [i32],
    ) -> Vec<i32> {
        let mut count: Vec<i32> = vec![0; 26];
        for next in connection[n].iter() {
            if !visits[*next] {
                visits[*next] = true;
                let t: Vec<i32> =
                    Solution::count_sub_trees_loop(*next, connection, bytes, visits, result);
                for i in 0..26 {
                    count[i] += t[i];
                }
            }
        }
        count[bytes[n] as usize - 97] += 1;
        result[n] = count[bytes[n] as usize - 97];
        count
    }

    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let len: usize = n as usize;
        let mut connection: Vec<Vec<usize>> = vec![Vec::new(); len];
        for edge in edges.iter() {
            connection[edge[0] as usize].push(edge[1] as usize);
            connection[edge[1] as usize].push(edge[0] as usize);
        }
        let bytes: &[u8] = labels.as_bytes();
        let mut visits: Vec<bool> = vec![false; len];
        visits[0] = true;
        let mut result: Vec<i32> = vec![0; len];
        Solution::count_sub_trees_loop(0, &connection, bytes, &mut visits, &mut result);
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut labels: String = String::new();
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => labels = arg,
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if tmp_row.len() == 2 {
                    edges.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == n || labels.len() != n as usize || 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::count_sub_trees(n, edges, labels);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
