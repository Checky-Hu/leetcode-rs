use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for (i, equation) in equations.iter().enumerate() {
            match map.get_mut(&equation[0]) {
                Some(x) => {
                    x.insert(equation[0].clone(), 1_f64);
                    x.insert(equation[1].clone(), values[i]);
                }
                None => {
                    let mut tmp: HashMap<String, f64> = HashMap::new();
                    tmp.insert(equation[0].clone(), 1_f64);
                    tmp.insert(equation[1].clone(), values[i]);
                    map.insert(equation[0].clone(), tmp);
                }
            }
            match map.get_mut(&equation[1]) {
                Some(x) => {
                    x.insert(equation[1].clone(), 1_f64);
                    x.insert(equation[0].clone(), 1_f64 / values[i]);
                }
                None => {
                    let mut tmp: HashMap<String, f64> = HashMap::new();
                    tmp.insert(equation[1].clone(), 1_f64);
                    tmp.insert(equation[0].clone(), 1_f64 / values[i]);
                    map.insert(equation[1].clone(), tmp);
                }
            }
        }
        let mut result: Vec<f64> = Vec::new();
        for query in queries.iter() {
            if !map.contains_key(&query[0]) || !map.contains_key(&query[1]) {
                result.push(-1_f64);
                continue;
            }
            let mut queue: Vec<(String, f64)> = Vec::new();
            queue.push((query[0].clone(), 1_f64));
            let mut visit: HashSet<String> = HashSet::new();
            visit.insert(query[0].clone());
            let mut found: bool = false;
            while !queue.is_empty() && !found {
                let mut next: Vec<(String, f64)> = Vec::new();
                for q in queue {
                    if q.0 == query[1] {
                        found = true;
                        result.push(q.1);
                        break;
                    }
                    if let Some(x) = map.get(&q.0) {
                        for (k, v) in x.iter() {
                            if !visit.contains(k) {
                                visit.insert(k.to_string());
                                next.push((k.to_string(), v * q.1));
                            }
                        }
                    }
                }
                if found {
                    break;
                } else {
                    queue = next;
                }
            }
            if !found {
                result.push(-1_f64);
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut equations: Vec<Vec<String>> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    let mut queries: Vec<Vec<String>> = Vec::new();
    let mut combine: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if equations.len() < len {
                    combine.push(arg);
                    if combine.len() == 2 {
                        equations.push(combine);
                        combine = Vec::new();
                    }
                } else if values.len() < len {
                    let value: f64 = f64::from_str(&arg).expect("Error parse.");
                    values.push(value);
                } else {
                    combine.push(arg);
                    if combine.len() == 2 {
                        queries.push(combine);
                        combine = Vec::new();
                    }
                }
            }
        }
    }

    if 0 == len || 3 * len + 1 > ret {
        println!("Require at least (1 + 3 * arg1) parameters.");
        return;
    }

    let result: Vec<f64> = Solution::calc_equation(equations, values, queries);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
