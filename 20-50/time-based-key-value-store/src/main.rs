use std::collections::HashMap;

struct TimeMap {
    k_t_map_: HashMap<String, Vec<i32>>,
    t_v_map_: HashMap<i32, String>,
}

impl TimeMap {
    fn new() -> Self {
        TimeMap {
            k_t_map_: HashMap::new(),
            t_v_map_: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        match self.k_t_map_.get_mut(&key) {
            Some(x) => x.push(timestamp),
            None => {
                self.k_t_map_.insert(key, vec![timestamp]);
            }
        }
        self.t_v_map_.insert(timestamp, value);
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.k_t_map_.get(&key) {
            Some(x) => match x.binary_search(&timestamp) {
                Ok(y) => self.t_v_map_.get(&x[y]).unwrap().to_string(),
                Err(e) => {
                    if e == 0 {
                        String::new()
                    } else {
                        self.t_v_map_.get(&x[e - 1]).unwrap().to_string()
                    }
                }
            },
            None => String::new(),
        }
    }
}

fn main() {
    let mut obj: TimeMap = TimeMap::new();
    let love: String = "love".to_string();
    obj.set(love.clone(), "high".to_string(), 10);
    obj.set(love.clone(), "low".to_string(), 20);
    println!("get love 5: {}", obj.get(love.clone(), 5));
    println!("get love 10: {}", obj.get(love.clone(), 10));
    println!("get love 15: {}", obj.get(love.clone(), 15));
    println!("get love 20: {}", obj.get(love.clone(), 20));
    println!("get love 25: {}", obj.get(love, 25));
}
