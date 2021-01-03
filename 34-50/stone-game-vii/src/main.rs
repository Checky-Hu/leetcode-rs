use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn stone_game_vii_loop(
        sum: i32,
        memo: &mut Vec<Vec<i32>>,
        stones: &[i32],
        left: usize,
        right: usize,
        is_alice: bool,
    ) -> i32 {
        if left == right {
            return 0;
        }
        if memo[left][right] > -1 {
            return memo[left][right];
        }
        let mut t: i32 = sum - stones[left];
        let r1: i32 = if is_alice { t } else { 0 - t }
            + Solution::stone_game_vii_loop(t, memo, &stones, left + 1, right, !is_alice);
        t = sum - stones[right];
        let r2: i32 = if is_alice { t } else { 0 - t }
            + Solution::stone_game_vii_loop(t, memo, &stones, left, right - 1, !is_alice);
        let (less, greater) = if r1 > r2 { (r2, r1) } else { (r1, r2) };
        if is_alice {
            memo[left][right] = greater;
            greater
        } else {
            memo[left][right] = less;
            less
        }
    }

    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let len: usize = stones.len();
        let mut sum: i32 = 0;
        for v in stones.iter() {
            sum += *v;
        }
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; len]; len];
        Solution::stone_game_vii_loop(sum, &mut memo, &stones, 0, len - 1, true)
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut stones: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let stone: i32 = i32::from_str(&arg).expect("Error parse.");
                stones.push(stone);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Stone game VII: {}", Solution::stone_game_vii(stones));
}
