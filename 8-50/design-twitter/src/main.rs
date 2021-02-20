use std::collections::HashMap;
use std::collections::HashSet;

struct Twitter {
    users_: HashMap<i32, HashSet<i32>>,
    tweets_: Vec<(i32, i32)>,
}

impl Twitter {
    fn new() -> Self {
        Twitter {
            users_: HashMap::new(),
            tweets_: Vec::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets_.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if self.tweets_.is_empty() {
            return Vec::new();
        }
        let mut result: Vec<i32> = Vec::with_capacity(10);
        let mut index: usize = self.tweets_.len() - 1;
        loop {
            if self.tweets_[index].0 == user_id
                || match self.users_.get(&user_id) {
                    Some(x) => x.contains(&self.tweets_[index].0),
                    None => false,
                }
            {
                result.push(self.tweets_[index].1);
                if result.len() == 10 {
                    break;
                }
            }
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.users_.get_mut(&follower_id) {
            Some(x) => {
                x.insert(followee_id);
            }
            None => {
                let mut set: HashSet<i32> = HashSet::new();
                set.insert(followee_id);
                self.users_.insert(follower_id, set);
            }
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(x) = self.users_.get_mut(&follower_id) {
            x.remove(&followee_id);
        }
    }
}

fn main() {
    let mut obj: Twitter = Twitter::new();
    obj.post_tweet(1, 5);
    let mut result = obj.get_news_feed(1);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
    // Test follow.
    obj.follow(1, 2);
    obj.post_tweet(2, 6);
    result = obj.get_news_feed(1);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
    // Test unfollow.
    obj.unfollow(1, 2);
    result = obj.get_news_feed(1);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
