use std::collections::HashMap;

struct MagicDictionary {
    map_: HashMap<usize, Vec<String>>,
}

impl MagicDictionary {
    fn new() -> Self {
        MagicDictionary {
            map_: HashMap::new(),
        }
    }

    fn build_dict(&mut self, dict: Vec<String>) {
        for word in dict {
            let len: usize = word.len();
            match self.map_.get_mut(&len) {
                Some(x) => (*x).push(word),
                None => {
                    self.map_.insert(len, vec![word]);
                }
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let len: usize = word.len();
        match self.map_.get(&len) {
            Some(x) => {
                let mut found: bool = false;
                for s in &(*x) {
                    let mut count: i32 = 0;
                    let w_bytes: &[u8] = word.as_bytes();
                    let s_bytes: &[u8] = s.as_bytes();
                    for i in 0..len {
                        if w_bytes[i] == s_bytes[i] {
                            continue;
                        } else {
                            count += 1;
                            if count > 1 {
                                break;
                            }
                        }
                    }
                    if count == 1 {
                        found = true;
                        break;
                    }
                }
                found
            }
            None => false,
        }
    }
}

fn main() {
    let mut obj: MagicDictionary = MagicDictionary::new();
    let hello: String = "hello".to_string();
    let leetcode: String = "leetcode".to_string();
    obj.build_dict(vec![hello.clone(), leetcode]);
    println!("Search {}: {}", hello.clone(), obj.search(hello));
    let hhllo: String = "hhllo".to_string();
    println!("Search {}: {}", hhllo.clone(), obj.search(hhllo));
    let hell: String = "hell".to_string();
    println!("Search {}: {}", hell.clone(), obj.search(hell));
    let leetcoded: String = "leetcoded".to_string();
    println!("Search {}: {}", leetcoded.clone(), obj.search(leetcoded));
}
