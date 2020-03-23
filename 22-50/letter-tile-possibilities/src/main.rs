use std::env;

struct Solution {}

impl Solution {
    fn helper(count: &mut Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        for i in 0..26 {
            if count[i] == 0 {
                continue;
            }
            sum += 1;
            count[i] -= 1;
            sum += Solution::helper(count);
            count[i] += 1;
        }
        sum
    }

    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut count: Vec<i32> = vec![0; 26];
        for u in tiles.as_bytes().iter() {
            count[*u as usize - 65] += 1;
        }
        Solution::helper(&mut count)
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let tiles: String = arg;
            println!(
                "Tile possibilities: {}",
                Solution::num_tile_possibilities(tiles)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
