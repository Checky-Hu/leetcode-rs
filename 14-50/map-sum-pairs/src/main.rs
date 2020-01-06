use std::collections::HashMap;

struct MapSum {
    map_: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        MapSum {
            map_: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.map_.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let p_bytes: &[u8] = prefix.as_bytes();
        let p_len: usize = p_bytes.len();
        let mut result: i32 = 0;
        for (k, v) in self.map_.iter() {
            let len: usize = k.len();
            if len < p_len {
                continue;
            }
            let mut is_match: bool = true;
            let bytes: &[u8] = k.as_bytes();
            for i in 0..p_len {
                if p_bytes[i] != bytes[i] {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                result += *v;
            }
        }
        result
    }
}

fn main() {
    let mut obj: MapSum = MapSum::new();
    obj.insert("apple".to_string(), 3);
    let ap: String = "ap".to_string();
    println!("Sum {}: {}", ap.clone(), obj.sum(ap.clone()));
    obj.insert("app".to_string(), 2);
    println!("Sum {}: {}", ap.clone(), obj.sum(ap));
}
