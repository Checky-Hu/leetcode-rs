use std::collections::HashSet;
use std::env;

struct Solution {
}

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut gene_set: HashSet<String> = bank.into_iter().collect();
        if !gene_set.contains(&end) {
            return -1
        }
        let gene_c: Vec<u8> = vec![65, 67, 71, 84];
        let mut queue: Vec<String> = vec![start];
        let mut result: i32 = 0;
        while !queue.is_empty() {
            let q_len: usize = queue.len();
            for _i in 0..q_len {
                let mut gene: String = queue.remove(0);
                if gene == end {
                    return result
                } else {
                    let g_len: usize = gene.len();
                    for j in 0..g_len {
                        unsafe {
                            let new_gene: &mut Vec<u8> = gene.as_mut_vec();
                            let original_c: u8 = new_gene[j];
                            for c in &gene_c {
                                if *c != original_c {
                                    new_gene[j] = *c;
                                    let tmp: String = String::from_utf8(new_gene.to_vec()).unwrap();
                                    if gene_set.contains(&tmp) {
                                        queue.push(tmp.clone());
                                        gene_set.remove(&tmp);
                                    }
                                }
                            }
                            new_gene[j] = original_c;
                        }
                    }
                }
            }
            result += 1;
        }
        -1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut start: String = String::new();
    let mut end: String = String::new();
    let mut bank: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => start = arg,
            2 => end = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                bank.push(s);
            },
        }
    }

    if ret == 0 {
        println!("Require at least 3 parameters.");
        return;
    }

    println!("Min mutation: {}", Solution::min_mutation(start, end, bank));
}
