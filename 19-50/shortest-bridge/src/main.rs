use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shortest_bridge(a: Vec<Vec<i32>>) -> i32 {
        let len: usize = a.len();
        let mut start: (usize, usize) = (0, 0);
        for (i, row) in a.iter().enumerate() {
            let mut found: bool = false;
            for (j, v) in row.iter().enumerate() {
                if *v == 1 {
                    start.0 = i;
                    start.1 = j;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        let mut set1: HashSet<(usize, usize)> = HashSet::new();
        set1.insert(start);
        let mut queue: Vec<(usize, usize)> = Vec::new();
        queue.push(start);
        while !queue.is_empty() {
            let mut tmp: Vec<(usize, usize)> = Vec::new();
            for p in queue {
                if p.0 > 0 && a[p.0 - 1][p.1] == 1 {
                    let t: (usize, usize) = (p.0 - 1, p.1);
                    if !set1.contains(&t) {
                        set1.insert(t);
                        tmp.push(t);
                    }
                }
                if p.0 < len - 1 && a[p.0 + 1][p.1] == 1 {
                    let t: (usize, usize) = (p.0 + 1, p.1);
                    if !set1.contains(&t) {
                        set1.insert(t);
                        tmp.push(t);
                    }
                }
                if p.1 > 0 && a[p.0][p.1 - 1] == 1 {
                    let t: (usize, usize) = (p.0, p.1 - 1);
                    if !set1.contains(&t) {
                        set1.insert(t);
                        tmp.push(t);
                    }
                }
                if p.1 < len - 1 && a[p.0][p.1 + 1] == 1 {
                    let t: (usize, usize) = (p.0, p.1 + 1);
                    if !set1.contains(&t) {
                        set1.insert(t);
                        tmp.push(t);
                    }
                }
            }
            queue = tmp;
        }
        let mut result: i32 = 0;
        let mut set2: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let mut reach: bool = false;
            let mut tmp: HashSet<(usize, usize)> = HashSet::new();
            let set: &HashSet<(usize, usize)> = if result == 0 { &set1 } else { &set2 };
            for p in set.iter() {
                if p.0 > 0 {
                    let t: (usize, usize) = (p.0 - 1, p.1);
                    if a[t.0][t.1] == 0 {
                        if !tmp.contains(&t) {
                            tmp.insert(t);
                        }
                    } else if !set1.contains(&t) {
                        reach = true;
                        break;
                    }
                }
                if p.0 < len - 1 {
                    let t: (usize, usize) = (p.0 + 1, p.1);
                    if a[t.0][t.1] == 0 {
                        if !tmp.contains(&t) {
                            tmp.insert(t);
                        }
                    } else if !set1.contains(&t) {
                        reach = true;
                        break;
                    }
                }
                if p.1 > 0 {
                    let t: (usize, usize) = (p.0, p.1 - 1);
                    if a[t.0][t.1] == 0 {
                        if !tmp.contains(&t) {
                            tmp.insert(t);
                        }
                    } else if !set1.contains(&t) {
                        reach = true;
                        break;
                    }
                }
                if p.1 < len - 1 {
                    let t: (usize, usize) = (p.0, p.1 + 1);
                    if a[t.0][t.1] == 0 {
                        if !tmp.contains(&t) {
                            tmp.insert(t);
                        }
                    } else if !set1.contains(&t) {
                        reach = true;
                        break;
                    }
                }
            }
            if reach {
                break;
            } else {
                result += 1;
                set2 = tmp;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == n as usize {
                    a.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == n || n * n > ret {
        println!("Require at least (1 + arg1 ^ 2) parameters.");
        return;
    }

    println!("Shortest bridge: {}", Solution::shortest_bridge(a));
}
