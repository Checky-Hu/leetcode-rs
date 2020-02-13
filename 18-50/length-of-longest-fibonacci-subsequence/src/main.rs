use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn len_longest_fib_subseq(a: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for v in a.iter() {
            set.insert(*v);
        }
        let mut result: i32 = 0;
        let len: usize = a.len();
        for i in 0..(len - 2) {
            for j in (i + 1)..(len - 1) {
                let mut count: i32 = 2;
                let mut n1: i32 = a[i];
                let mut n2: i32 = a[j];
                while set.contains(&(n1 + n2)) {
                    count += 1;
                    let t: i32 = n2;
                    n2 += n1;
                    n1 = t;
                }
                if count >= 3 && count > result {
                    result = count;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Length of longest fibonacci subsequence: {}",
        Solution::len_longest_fib_subseq(a)
    );
}
