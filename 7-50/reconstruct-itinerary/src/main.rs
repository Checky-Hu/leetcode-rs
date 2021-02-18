use std::collections::HashMap;
use std::env;

struct Solution {}

impl Solution {
    fn find_itinerary_loop(
        from: &str,
        ticket_map: &HashMap<String, Vec<String>>,
        visits_map: &mut HashMap<(String, String), i32>,
        result: &mut Vec<String>,
        length: usize,
        target: usize,
    ) -> bool {
        if length == target {
            return true;
        }
        if let Some(x) = ticket_map.get(from) {
            for to in x.iter() {
                let visit: (String, String) = (from.to_string(), to.clone());
                let count: i32 = match visits_map.get(&visit) {
                    Some(y) => *y,
                    None => 0,
                };
                if count > 0 {
                    if let Some(y) = visits_map.get_mut(&visit) {
                        *y -= 1;
                    }
                    result.push(to.clone());
                    if Solution::find_itinerary_loop(
                        to,
                        ticket_map,
                        visits_map,
                        result,
                        length + 1,
                        target,
                    ) {
                        return true;
                    } else {
                        result.pop();
                        if let Some(y) = visits_map.get_mut(&visit) {
                            *y += 1;
                        }
                    }
                }
            }
            false
        } else {
            false
        }
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let target: usize = tickets.len();
        let mut visits_map: HashMap<(String, String), i32> = HashMap::new();
        let mut ticket_map: HashMap<String, Vec<String>> = HashMap::new();
        for mut ticket in tickets {
            let to: String = ticket.pop().unwrap();
            let from: String = ticket.pop().unwrap();
            let visit: (String, String) = (from.clone(), to.clone());
            match visits_map.get_mut(&visit) {
                Some(x) => *x += 1,
                None => {
                    visits_map.insert(visit, 1);
                }
            }
            match ticket_map.get_mut(&from) {
                Some(x) => x.push(to),
                None => {
                    ticket_map.insert(from, vec![to]);
                }
            }
        }
        for v in ticket_map.values_mut() {
            v.sort();
        }
        let mut result: Vec<String> = Vec::new();
        let source: String = "JFK".to_string();
        result.push(source.clone());
        Solution::find_itinerary_loop(
            &source,
            &ticket_map,
            &mut visits_map,
            &mut result,
            0,
            target,
        );
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut tickets: Vec<Vec<String>> = Vec::new();
    let mut ticket: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                ticket.push(arg);
                if ticket.len() == 2 {
                    tickets.push(ticket);
                    ticket = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result = Solution::find_itinerary(tickets);
    for r in result.iter() {
        println!("{} ", *r);
    }
}
