// main.rs

mod b_tree;
use b_tree::BST;

fn main() {
    let mut memory_db = BST::new();

    memory_db.put(1, "sam".to_string());
    println!("{:?}", memory_db.get(1)); 
     // Overwriting the value for key 1
    memory_db.put(1, "sammy".to_string());
    memory_db.put(2, "samantha".to_string());
    memory_db.put(3, "john samuel".to_string());
    memory_db.put(4, "aravind raj".to_string());
   
    println!("printing values on key order"); 
    memory_db.print_in_order(); 
}
