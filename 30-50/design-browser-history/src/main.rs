struct BrowserHistory {
    history: Vec<String>,
    index: usize,
    end: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            index: 0,
            end: 0,
        }
    }

    fn visit(&mut self, url: String) {
        self.index += 1;
        self.end = self.index;
        if self.history.len() > self.index {
            self.history[self.index] = url;
        } else {
            self.history.push(url);
        }
    }

    fn back(&mut self, steps: i32) -> String {
        let t: usize = steps as usize;
        if self.index >= t {
            self.index -= t;
        } else {
            self.index = 0;
        }
        self.history[self.index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        let t: usize = steps as usize;
        if self.index + t <= self.end {
            self.index += t;
        } else {
            self.index = self.end;
        }
        self.history[self.index].clone()
    }
}

fn main() {
    let mut obj = BrowserHistory::new("leetcode.com".to_string());
    obj.visit("google.com".to_string());
    obj.visit("facebook.com".to_string());
    obj.visit("youtube.com".to_string());
    println!("back 1: {}", obj.back(1));
    println!("back 1: {}", obj.back(1));
    println!("forward 1: {}", obj.forward(1));
    obj.visit("linkedin.com".to_string());
    println!("forward 2: {}", obj.forward(2));
    println!("back 2: {}", obj.back(2));
    println!("back 7: {}", obj.back(7));
}
