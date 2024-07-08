use std::collections::HashMap;

#[derive(Debug)]
struct LRUCache {
    rank: usize,
    capacity: usize,
    key_to_value_with_rank: HashMap<i32, (usize, i32)>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            rank: 0,
            capacity: capacity as usize,
            key_to_value_with_rank: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match key {
            key if self.key_to_value_with_rank.contains_key(&key) => {
                let rank_change = self.rank;
                self.key_to_value_with_rank
                    .entry(key)
                    .and_modify(|pair| pair.0 = rank_change);
                self.rank += 1;
                return self.key_to_value_with_rank.get(&key).unwrap().1;
            }
            _ => {
                return -1;
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.capacity {
            capacity if capacity > self.key_to_value_with_rank.len() => {
                self.key_to_value_with_rank.insert(key, (self.rank, value));
                self.rank += 1;
            }
            _ => {
                self.key_to_value_with_rank
                    .entry(key)
                    .and_modify(|pair| pair.1 = value);
            }
        }
    }
}

fn main() {
    let mut container1 = LRUCache::new(2);
    container1.put(1, 101);
    println!("{:?}", container1);
    container1.put(2, 102);
    println!("{:?}", container1);
    container1.get(1);
    println!("{:?}", container1);
    //container1.put(3, 133);
    //container1.put(3, 133);
    // container1.key_to_value_with_rank.insert(4, (1, 133));
    // container1.key_to_value_with_rank.insert(5, (1, 133));
    // container1.key_to_value_with_rank.insert(6, (1, 133));
}
