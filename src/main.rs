struct LRUCache {
    capacity: usize,
    data: Vec<(i32, i32)>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            data: Vec::with_capacity(capacity as usize),
        }
    }

    fn put(&self, key: i32, value: i32) {}
}

fn main() {
    let container1 = LRUCache::new(10);
    let container2 = LRUCache::new(20);
    let container3 = LRUCache::new(30);

    // println!(
    //     "The capacity of the container1 is {:?}",
    //     container1.capacity
    // );
    // println!(
    //     "The capacity of the container2 is {:?}",
    //     container2.capacity
    // );
    // println!(
    //     "The capacity of the container3 is {:?}",
    //     container3.capacity
    // );
}
