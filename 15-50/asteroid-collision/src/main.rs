use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = asteroids;
        loop {
            let mut is_collision: bool = false;
            for i in 1..result.len() {
                if result[i - 1] > 0 && result[i] < 0 {
                    match 0.cmp(&(result[i - 1] + result[i])) {
                        Ordering::Less => {
                            result.remove(i);
                        }
                        Ordering::Equal => {
                            result.remove(i - 1);
                            result.remove(i - 1);
                        }
                        Ordering::Greater => {
                            result.remove(i - 1);
                        }
                    }
                    is_collision = true;
                    break;
                }
            }
            if !is_collision {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut asteroids: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let asteroid: i32 = i32::from_str(&arg).expect("Error parse.");
                asteroids.push(asteroid);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::asteroid_collision(asteroids);
    for r in &result {
        print!("{} ", r);
    }
    println!();
}
