use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shopping_offers_loop(
        price: &[i32],
        special: &[Vec<i32>],
        target: &mut Vec<i32>,
        len: usize,
    ) -> i32 {
        let mut result: i32 = 0;
        for i in 0..len {
            result += price[i] * target[i];
        }
        for s in special {
            let mut is_valid: bool = true;
            for i in 0..len {
                if s[i] > target[i] {
                    is_valid = false;
                }
                target[i] -= s[i];
            }
            if is_valid {
                let r: i32 = Solution::shopping_offers_loop(price, special, target, len) + s[len];
                if r < result {
                    result = r;
                }
            }
            for i in 0..len {
                target[i] += s[i];
            }
        }
        result
    }

    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let len: usize = price.len();
        let mut target: Vec<i32> = needs;
        Solution::shopping_offers_loop(&price, &special, &mut target, len)
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut count: i32 = 0;
    let mut price: Vec<i32> = Vec::new();
    let mut needs: Vec<i32> = Vec::new();
    let mut special: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => count = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if price.len() < count as usize {
                    price.push(n);
                } else if needs.len() < count as usize {
                    needs.push(n);
                } else {
                    tmp.push(n);
                    if tmp.len() == count as usize + 1 {
                        special.push(tmp);
                        tmp = Vec::new();
                    }
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (2 * arg1 + 1) parameters.");
        return;
    }

    println!(
        "Least cost: {}",
        Solution::shopping_offers(price, special, needs)
    );
}
