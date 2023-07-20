use std::sync::{Arc, Mutex};
use std::thread;

// Define the database record structure
#[derive(Debug)]
#[derive(Clone)]
struct Record {
    id: u32,
    value: String,
}

// Define the in-memory database
struct Database {
    records: Vec<Record>,
}

impl Database {
    fn new() -> Self {
        Database { records: Vec::new() }
    }

    fn insert(&mut self, record: Record) {
        self.records.push(record);
    }

    fn get_all(&self) -> Vec<Record> {
        self.records.clone()
    }
}

fn main() {
    // Create the shared database data structure
    let database = Arc::new(Mutex::new(Database::new()));

    // Spawn multiple threads to perform database operations
    let mut handles = Vec::new();
    for i in 0..5 {
        let database = Arc::clone(&database);
        let handle = thread::spawn(move || {
            // Perform database operations within each thread
            let mut db = database.lock().unwrap();
            let record = Record {
                id: i,
                value: format!("Value {}", i),
            };
            db.insert(record);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Access the database and print the records
    let db = database.lock().unwrap();
    let records = db.get_all();
    for record in records {
        println!("{:?}", record);
    }
}
