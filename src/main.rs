use std::collections::HashMap;

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
                // self.rank_to_value.insert(self.rank, key);
                self.rank += 1;
            }
            _ => {
                if self.key_to_value_with_rank.contains_key(&key) {
                    self.key_to_value_with_rank.insert(key, (self.rank, value));
                    self.rank += 1;
                } else {
                    let key_with_min_rank = *self
                        .key_to_value_with_rank
                        .iter()
                        .min_by_key(|k| k.1)
                        .unwrap()
                        .0;
                    self.key_to_value_with_rank.remove(&key_with_min_rank);
                    self.key_to_value_with_rank.insert(key, (self.rank, value));
                    self.rank += 1;
                }
            }
        }
    }
}

fn main() {
    // Test case 1 ///
    // let mut container1 = LRUCache::new(3);
    // container1.put(12, 101); //rank 0
    // container1.put(2, 102); // rank 1
    // container1.put(2, 222); // rank 2
    // container1.put(1, 777); // rank 3
    // container1.put(3, 999); //rank 4
    // container1.put(3, 222);
    // container1.put(3, 111);
    // container1.put(3, 12);
    // container1.put(5, 13);
    // container1.put(17, 1777);

    // Test case 2 //
    let mut container1 = LRUCache::new(2);
    container1.put(12, 101); //rank 0
    container1.put(2, 102); // rank 1
    container1.get(2);
    container1.get(23);
    container1.put(5, 555);
}
