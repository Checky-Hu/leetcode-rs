use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn find_group(group: &[usize], n: usize) -> usize {
        let mut target: usize = n;
        while group[target] != target {
            target = group[target];
        }
        target
    }

    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let len: usize = s.len();
        let mut group: Vec<usize> = Vec::with_capacity(len);
        for i in 0..len {
            group.push(i);
        }
        for pair in pairs.iter() {
            let g1: usize = Solution::find_group(&group, pair[0] as usize);
            let g2: usize = Solution::find_group(&group, pair[1] as usize);
            let (less, greater) = if g2 > g1 { (g1, g2) } else { (g2, g1) };
            group[greater] = less;
            group[pair[0] as usize] = less;
            group[pair[1] as usize] = less;
        }
        loop {
            let mut has_change: bool = false;
            for i in 0..len {
                let g: usize = Solution::find_group(&group, group[i]);
                if g != group[i] {
                    group[i] = g;
                    has_change = true;
                }
            }
            if !has_change {
                break;
            }
        }
        // <group, (characters, index)>
        let mut map: HashMap<usize, (Vec<u8>, usize)> = HashMap::new();
        let s_bytes: &[u8] = s.as_bytes();
        for (i, u) in s_bytes.iter().enumerate() {
            match map.get_mut(&group[i]) {
                Some(x) => x.0.push(*u),
                None => {
                    map.insert(group[i], (vec![*u], 0));
                }
            }
        }
        for v in map.values_mut() {
            v.0.sort();
        }
        let mut result: String = String::with_capacity(len);
        for g in group.iter() {
            if let Some(x) = map.get_mut(g) {
                result.push(x.0[x.1] as char);
                x.1 += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut pairs: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    pairs.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Smallest string with swaps: {}",
        Solution::smallest_string_with_swaps(s, pairs)
    );
}
