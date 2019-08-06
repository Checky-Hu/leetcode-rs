struct RecentCounter {
    stack_: Vec<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            stack_: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.stack_.push(t);
        loop {
            if self.stack_.is_empty() {
                break;
            } else {
                if t - self.stack_[0] > 3000 {
                    self.stack_.remove(0);
                } else {
                    break;
                }
            }
        }
        self.stack_.len() as i32
    }
}

fn main() {
    let mut obj: RecentCounter = RecentCounter::new();
    println!("Count: {}", obj.ping(1));
    println!("Count: {}", obj.ping(100));
    println!("Count: {}", obj.ping(3001));
    println!("Count: {}", obj.ping(3002));
}
