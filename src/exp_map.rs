use std::collections::HashMap;
extern crate chrono;
use chrono::prelude::*;

pub struct ExpMap {
    pub requests: HashMap<String, Entry>
}

struct Entry {
    timestamp: i64,
    count: u8
}

const EXPIRY_TIME: i32 = 30;

impl ExpMap {
    pub fn new() -> Self {
        Self {
            requests: HashMap::<String, Entry>::new()
        }
    }

    pub fn incr(&mut self, key: String) -> u8 {
        let now = Utc::now().timestamp();

        self.requests.entry(key).and_modify(|entry| {;
            if now - entry.timestamp > EXPIRY_TIME.into() {
                entry.count = 0;
                entry.timestamp = now;
            } else {
                entry.count += 1;
            }
        }).or_insert(Entry {
            timestamp: now,
            count: 1
        }).count
    }
}
