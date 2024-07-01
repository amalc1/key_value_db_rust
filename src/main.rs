use std::collections::BTreeMap;

#[derive(Debug)]
struct MemoryDb {
    data:  BTreeMap<i32, String>,
}

impl MemoryDb {
    // Constructor method to create a new MemoryDb instance
    fn new() -> Self {
        MemoryDb {
            data: BTreeMap::new(),
        }
    }

    // Method to put a key-value pair into the database
    fn put(&mut self, key: i32, value: &str) {
        

     // Insert or update the key-value pair
     self.data.insert(key, value.to_string());   

}

     // Method to get the value associated with a key from the database
     fn get(&self, key: i32) -> Option<&str> {

        // Retrieve the value for the key
        self.data.get(&key).map(|s| s.as_str())
    }

}

fn main() {
    let mut db = MemoryDb::new();

    db.put(1, "sam");
    println!("{:?}", db.get(1)); // Output: Some("sam")
    // Overwriting the value for key 1
    db.put(1, "sammy");
    db.put(2, "samantha");
    println!("{:?}", db.get(1)); // Output: Some("sammy")
    println!("{:?}", db.get(2)); // Output: Some("samantha")
}
