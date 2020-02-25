use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn helper(n: i32, memo: &mut HashMap<i32, Vec<i32>>) -> &Vec<i32> {
        if !memo.contains_key(&n) {
            let mut result: Vec<i32> = Vec::new();
            for v in Solution::helper((n + 1) / 2, memo) {
                result.push(2 * *v - 1);
            }
            for v in Solution::helper(n / 2, memo) {
                result.push(2 * *v);
            }
            memo.insert(n, result);
        }
        memo.get(&n).unwrap()
    }

    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut memo: HashMap<i32, Vec<i32>> = HashMap::new();
        memo.insert(1, vec![1]);
        let result: &Vec<i32> = Solution::helper(n, &mut memo);
        result.to_vec()
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            let result: Vec<i32> = Solution::beautiful_array(n);
            for r in result.iter() {
                print!("{} ", *r);
            }
            println!();
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
