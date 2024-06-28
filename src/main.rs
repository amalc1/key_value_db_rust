#[derive(Debug)]
struct MemoryDb {
    data: Vec<KeyValue>,
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

impl MemoryDb {
    // Constructor method to create a new MemoryDb instance
    fn new() -> Self {
        MemoryDb {
            data: Vec::new(),
        }
    }

    // Method to put a key-value pair into the database
    fn put(&mut self, key: &str, value: &str) {
        // Check if key or value is empty
        if key.is_empty() {
            eprintln!("Error: Key cannot be empty.");
            return;
        }

        // Check if the key already exists
         match self.data.iter_mut().find(|item| item.key == key) {
            Some(existing) => existing.value = value.to_string(), // Update existing value
            None => self.data.push(KeyValue {
                key: key.to_string(),
                value: value.to_string(),
            }), // Add new key-value pair
        }
    }

    // Method to get the value associated with a key from the database
    fn get(&self, key: &str) -> Option<&str> {
        // Check if key is empty    
        if key.is_empty() {
            eprintln!("Error: Key cannot be empty.");
            return None;
        }

        // Search for the key in the database
        match self.data.iter().find(|item| item.key == key) {
            Some(item) => Some(&item.value),
            None => {
                eprintln!("Error: Key doesn't exist in db.");
                None
            }
        }
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
