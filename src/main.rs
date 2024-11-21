use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let hash = Self::calculate_hash(index, timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Chain should have at least one block");
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.data,
                &current_block.previous_hash,
            );

            if current_block.hash != recalculated_hash {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    println!("Genesis Block: {:?}", blockchain.chain[0]);

    blockchain.add_block("Block 1: Hello, Blockchain!".to_string());
    blockchain.add_block("Block 2: Learning Rust is fun.".to_string());
    blockchain.add_block("Block 3: Blockchain Demo.".to_string());

    println!("\nBlockchain:");
    for block in &blockchain.chain {
        println!("{:?}", block);
    }

    println!("\nIs blockchain valid? {}", blockchain.is_valid());
}
