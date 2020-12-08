use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_mut: Vec<i32> = nums;
        let mut count: Vec<i32> = vec![0; 201];
        for n in nums_mut.iter() {
            count[(*n + 100) as usize] += 1;
        }
        nums_mut.sort_by(
            |a, b| match count[(a + 100) as usize].cmp(&count[(b + 100) as usize]) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => b.cmp(&a),
                Ordering::Greater => Ordering::Greater,
            },
        );
        nums_mut
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    let result: Vec<i32> = Solution::frequency_sort(nums);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
