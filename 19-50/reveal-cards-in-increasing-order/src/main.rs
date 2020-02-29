use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(cur: Vec<i32>, swap: bool) -> Vec<i32> {
        let len: usize = cur.len();
        if len >= 2 {
            let mut result: Vec<i32> = vec![0; len];
            let mut next: Vec<i32> = Vec::new();
            let mut index: usize = 0;
            for (i, v) in cur.iter().enumerate().take(len) {
                if i < (len + 1) / 2 {
                    result[index] = *v;
                    index += 2;
                } else {
                    next.push(*v);
                }
            }
            let rest: Vec<i32> = Solution::helper(next, len & 1 == 1);
            index = 1;
            for v in rest.iter() {
                result[index] = *v;
                index += 2;
            }
            if swap {
                let last: i32 = result.pop().unwrap();
                result.insert(0, last);
            }
            result
        } else {
            cur
        }
    }

    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = deck;
        result.sort();
        Solution::helper(result, false)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut deck: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            deck.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::deck_revealed_increasing(deck);
    for n in &result {
        print!("{} ", *n);
    }
    println!();
}
