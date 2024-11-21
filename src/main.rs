use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Inicializamos el bloque sin hash y sin nonce
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };

        // Minamos el bloque
        block.mine_block(difficulty);

        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // El hash debe empezar con este número de ceros
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined with nonce {}: {}", self.nonce, self.hash);
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize, // Número de ceros requeridos
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().expect("Chain should have at least one block");
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
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

            let recalculated_hash = current_block.calculate_hash();
            if current_block.hash != recalculated_hash {
                return false;
            }

            // Validamos que el hash cumpla con la dificultad
            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4); // Dificultad: 4 ceros iniciales

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