use std::collections::HashMap;

struct LRUCache {
    count_: i32,
    size_: usize,
    // contains [key, value, count].
    queue_: Vec<Vec<i32>>,
    // key <-> index in queue_.
    map_: HashMap<i32, usize>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            count_: 0,
            size_: capacity as usize,
            queue_: Vec::new(),
            map_: HashMap::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self.map_.get(&key) {
            Some(x) => {
                self.queue_[*x][2] = self.count_;
                self.count_ += 1;
                self.queue_[*x][1]
            },
            None => -1,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.map_.get(&key) {
            Some(x) => {
                self.queue_[*x][1] = value;
                self.queue_[*x][2] = self.count_;
                self.count_ += 1;
            },
            None => {
                if self.queue_.len() == self.size_ {
                    // find index for new value.
                    let mut min_i: usize = 0;
                    let mut i: usize = 1;
                    while i < self.size_ {
                        if self.queue_[i][2] < self.queue_[min_i][2] {
                            min_i = i;
                        }
                        i += 1;
                    }
                    self.map_.remove(&self.queue_[min_i][0]);
                    self.queue_[min_i][0] = key;
                    self.queue_[min_i][1] = value;
                    self.queue_[min_i][2] = self.count_;
                    self.count_ += 1;
                    self.map_.insert(key, min_i);
                } else {
                    self.queue_.push(vec![key, value, self.count_]);
                    self.count_ += 1;
                    self.map_.insert(key, self.queue_.len() - 1);
                }
            },
        };
    }
}

fn main() {
    let mut obj: LRUCache = LRUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    println!("Get 1: {}", obj.get(1));
    obj.put(3, 3);
    println!("Get 2: {}", obj.get(2));
    obj.put(4, 4);
    println!("Get 1: {}", obj.get(1));
    println!("Get 3: {}", obj.get(3));
    println!("Get 4: {}", obj.get(4));
}
