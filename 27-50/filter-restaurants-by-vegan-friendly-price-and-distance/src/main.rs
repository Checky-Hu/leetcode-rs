use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut rest: Vec<Vec<i32>> = Vec::new();
        for restaurant in restaurants.iter() {
            if vegan_friendly == 1 && restaurant[2] != vegan_friendly {
                continue;
            }
            if restaurant[3] > max_price || restaurant[4] > max_distance {
                continue;
            }
            rest.push(vec![restaurant[0], restaurant[1]]);
        }
        rest.sort_by(|a, b| match a[1].cmp(&b[1]) {
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => b[0].cmp(&a[0]),
            Ordering::Greater => Ordering::Less,
        });
        let mut result: Vec<i32> = Vec::new();
        for r in rest.iter() {
            result.push(r[0]);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut vegan_friendly: i32 = 0;
    let mut max_price: i32 = 0;
    let mut max_distance: i32 = 0;
    let mut restaurants: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => vegan_friendly = i32::from_str(&arg).expect("Error parse."),
            2 => max_price = i32::from_str(&arg).expect("Error parse."),
            3 => max_distance = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 5 {
                    restaurants.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
    }

    let result: Vec<i32> =
        Solution::filter_restaurants(restaurants, vegan_friendly, max_price, max_distance);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
