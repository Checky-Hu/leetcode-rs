use std::collections::HashMap;

struct TweetCounts {
    tweets_: HashMap<String, Vec<i32>>,
}

impl TweetCounts {
    fn new() -> Self {
        TweetCounts {
            tweets_: HashMap::new(),
        }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        match self.tweets_.get_mut(&tweet_name) {
            Some(x) => {
                let mut pos: usize = x.len();
                for (i, v) in x.iter().enumerate() {
                    if *v > time {
                        pos = i;
                        break;
                    }
                }
                x.insert(pos, time);
            }
            None => {
                self.tweets_.insert(tweet_name, vec![time]);
            }
        }
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut s: i32 = start_time;
        let delta: i32 = match freq.as_str() {
            "minute" => 60,
            "hour" => 3600,
            "day" => 86400,
            _ => 0,
        };
        let mut index: usize = 0;
        while s <= end_time {
            let next: i32 = s + delta;
            let e: i32 = if next > end_time { end_time } else { next - 1 };
            match self.tweets_.get(&tweet_name) {
                Some(x) => {
                    let mut count: i32 = 0;
                    let mut i: usize = index;
                    while i < x.len() {
                        if x[i] >= s {
                            if x[i] <= e {
                                count += 1;
                            } else {
                                index = i;
                                break;
                            }
                        }
                        i += 1;
                    }
                    result.push(count)
                }
                None => result.push(0),
            }
            s = next;
        }
        result
    }
}

fn main() {
    let mut obj: TweetCounts = TweetCounts::new();
    obj.record_tweet("tweet3".to_string(), 0);
    obj.record_tweet("tweet3".to_string(), 60);
    obj.record_tweet("tweet3".to_string(), 10);
    let mut result: Vec<i32> =
        obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 59);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
    result = obj.get_tweet_counts_per_frequency("minute".to_string(), "tweet3".to_string(), 0, 60);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
    obj.record_tweet("tweet3".to_string(), 120);
    result = obj.get_tweet_counts_per_frequency("hour".to_string(), "tweet3".to_string(), 0, 210);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
