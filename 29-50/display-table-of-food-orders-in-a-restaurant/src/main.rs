use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

struct Solution {}

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let mut food_set: HashSet<String> = HashSet::new();
        let mut table_set: HashSet<String> = HashSet::new();
        for order in orders.iter() {
            if !table_set.contains(&order[1]) {
                table_set.insert(order[1].clone());
            }
            if !food_set.contains(&order[2]) {
                food_set.insert(order[2].clone());
            }
            match map.get_mut(&order[1]) {
                Some(x) => match x.get_mut(&order[2]) {
                    Some(y) => *y += 1,
                    None => {
                        x.insert(order[2].clone(), 1);
                    }
                },
                None => {
                    let mut t: HashMap<String, i32> = HashMap::new();
                    t.insert(order[2].clone(), 1);
                    map.insert(order[1].clone(), t);
                }
            }
        }
        let mut food: Vec<String> = Vec::new();
        for v in food_set.iter() {
            food.push(v.clone());
        }
        food.sort();
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut header: Vec<String> = vec!["Table".to_string()];
        for f in food.iter() {
            header.push(f.clone());
        }
        result.push(header);
        let mut table: Vec<String> = Vec::new();
        for v in table_set.iter() {
            table.push(v.clone());
        }
        table.sort_by(|a, b| a.parse::<i32>().unwrap().cmp(&b.parse::<i32>().unwrap()));
        for t in table.iter() {
            let mut row: Vec<String> = vec![t.clone()];
            let contains: &HashMap<String, i32> = map.get(t).unwrap();
            for f in food.iter() {
                match contains.get(f) {
                    Some(x) => row.push(x.to_string()),
                    None => row.push("0".to_string()),
                }
            }
            result.push(row);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut orders: Vec<Vec<String>> = Vec::new();
    let mut order: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                order.push(s);
                if order.len() == 3 {
                    orders.push(order);
                    order = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !order.is_empty() {
        println!("Require at least 3 parameters.");
        return;
    }

    let result: Vec<Vec<String>> = Solution::display_table(orders);
    for r in result.iter() {
        for s in r.iter() {
            print!("{} ", *s);
        }
        println!();
    }
}
