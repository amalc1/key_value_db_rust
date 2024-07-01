use std::collections::BTreeMap;

#[derive(Debug)]
struct MemoryDb {
    data:  BTreeMap<String, String>,
}

impl MemoryDb {
    // Constructor method to create a new MemoryDb instance
    fn new() -> Self {
        MemoryDb {
            data: BTreeMap::new(),
        }
    }

    // Method to put a key-value pair into the database
    fn put(&mut self, key: &str, value: &str) {
        // Check if key or value is empty
        if key.is_empty() {
            eprintln!("Error: Key cannot be empty.");
            return;
        }

     // Insert or update the key-value pair
     self.data.insert(key.to_string(), value.to_string());   

}

     // Method to get the value associated with a key from the database
     fn get(&self, key: &str) -> Option<&str> {
        // Check if key is empty    
        if key.is_empty() {
            eprintln!("Error: Key cannot be empty.");
            return None;
        }

        // Retrieve the value for the key
        self.data.get(key).map(|s| s.as_str())
    }

}

fn main() {
    let mut db = MemoryDb::new();

    db.put("name", "sam");
    println!("{:?}", db.get("name")); 
    // over writing name with new value sammy
    db.put("name", "sammy"); 
    db.put("name2", "samantha"); 
    println!("{:?}", db.get("name")); 
    println!("{:?}", db.get("name2")); 
}
