use sha2::{Sha256, Digest};
use std::io::{self, Write};

struct Bloomfilter {
    bloom: [bool; 1000],
}

impl Bloomfilter {
    pub fn new() -> Bloomfilter {
        Bloomfilter {
            bloom: [false; 1000]
        }        
    }

    fn hash_key(key: &String) -> (u16, u16, u16) {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let bytes = hasher.finalize();

        // take first 8 bytes as u64, map to 0..100
        let mut arr = [0u8; 8];
        arr.copy_from_slice(&bytes[..8]);
        let v = u64::from_be_bytes(arr);
        let hash1 = (v % 1001) as u16;

        arr.copy_from_slice(&bytes[8..16]);
        let v = u64::from_be_bytes(arr);
        let hash2 = (v % 1001) as u16;

        arr.copy_from_slice(&bytes[16..24]);
        let v = u64::from_be_bytes(arr);
        let hash3 = (v % 1001) as u16;

        (hash1, hash2, hash3)
    }

    pub fn add(&mut self, key: &String) {
        let (u1, u2, u3) = Bloomfilter::hash_key(key);

        self.bloom[u1 as usize] = true;
        self.bloom[u2 as usize] = true;
        self.bloom[u3 as usize] = true;
    }

    pub fn key_present(&self, key: &String) -> bool {
        let (u1, u2, u3) = Bloomfilter::hash_key(key);

        self.bloom[u1 as usize] && self.bloom[u2 as usize] && self.bloom[u3 as usize]
    }
}

fn prompt_user(bloomfilter: &mut Bloomfilter) {
    loop {
        println!("\nChoose an option:\n1. Add a key\n2. Check if a key exists\n3. Quit");
        print!("Enter choice (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read input. Try again.");
            continue;
        }

        match choice.trim() {
            "1" => {
                print!("Enter key to add: ");
                io::stdout().flush().unwrap();
                let mut key = String::new();
                if io::stdin().read_line(&mut key).is_err() {
                    eprintln!("Failed to read key.");
                    continue;
                }
                let key = key.trim().to_string();
                bloomfilter.add(&key);
                println!("Key '{}' added.", key);
            }
            "2" => {
                print!("Enter key to check: ");
                io::stdout().flush().unwrap();
                let mut key = String::new();
                if io::stdin().read_line(&mut key).is_err() {
                    eprintln!("Failed to read key.");
                    continue;
                }
                let key = key.trim().to_string();
                if bloomfilter.key_present(&key) {
                    println!("Key '{}' exists.", key);
                } else {
                    println!("Key '{}' not found.", key);
                }
            }
            "3" => {
                println!("Quitting.");
                break;
            }
            _ => {
                println!("Invalid choice. Enter 1, 2, or 3.");
            }
        }
    }
}

fn main() {
    let mut bloomfilter = Bloomfilter::new();
    prompt_user(&mut bloomfilter);    
}
