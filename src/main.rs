use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::fs::File;
use std::io::{Write, BufWriter};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    nonce: u64,
    hash: String,
    mining_time: u64, // Tiempo de minería en milisegundos
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
            mining_time: 0,
        };

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
        let target = "0".repeat(difficulty);
        let start_time = Instant::now();

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        let elapsed_time = start_time.elapsed();
        self.mining_time = elapsed_time.as_millis() as u64;

        println!(
            "Block mined with nonce {}: {} (Mining time: {} ms)",
            self.nonce, self.hash, self.mining_time
        );
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
    time_target: u64,
    adjustment_interval: usize,
    last_adjustment_time: u64,
}

impl Blockchain {
    fn new(difficulty: usize, time_target: u64, adjustment_interval: usize) -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
            time_target,
            adjustment_interval,
            last_adjustment_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        }
    }

    fn add_block(&mut self, data: String) {
        if self.chain.len() % self.adjustment_interval == 0 {
            self.adjust_difficulty();
        }

        let previous_block = self.chain.last().expect("Chain should have at least one block");
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
            self.difficulty,
        );
        self.chain.push(new_block);
    }

    fn adjust_difficulty(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let time_elapsed = current_time - self.last_adjustment_time;
        let expected_time = self.time_target * self.adjustment_interval as u64;

        if time_elapsed < expected_time {
            self.difficulty += 1;
        } else if self.difficulty > 1 {
            self.difficulty -= 1;
        }

        println!(
            "Difficulty adjusted to {}. Time elapsed: {} seconds (Expected: {} seconds)",
            self.difficulty, time_elapsed, expected_time
        );

        self.last_adjustment_time = current_time;
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

            if !current_block.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        true
    }

    fn average_mining_time(&self) -> f64 {
        let total_time: u64 = self.chain.iter().map(|block| block.mining_time).sum();
        total_time as f64 / self.chain.len() as f64
    }

    fn save_mining_stats(&self, filename: &str) {
        let file = File::create(filename).expect("Unable to create file");
        let mut writer = BufWriter::new(file);

        writer
            .write_all(b"Index,Mining Time (ms),Hash\n")
            .expect("Unable to write headers");

        for block in &self.chain {
            let line = format!("{},{},{}\n", block.index, block.mining_time, block.hash);
            writer.write_all(line.as_bytes()).expect("Unable to write data");
        }

        println!("Mining statistics saved to {}", filename);
    }
}

fn main() {
    let mut blockchain = Blockchain::new(4, 10, 3); // Dificultad inicial: 4, tiempo objetivo: 10s, ajuste cada 3 bloques

    blockchain.add_block("Block 1: Hello, Blockchain!".to_string());
    blockchain.add_block("Block 2: Learning Rust is fun.".to_string());
    blockchain.add_block("Block 3: Blockchain Demo.".to_string());
    blockchain.add_block("Block 4: Adjust Difficulty.".to_string());
    blockchain.add_block("Block 5: Another block.".to_string());

    println!("\nBlockchain:");
    for block in &blockchain.chain {
        println!(
            "Block {} mined in {} ms: Hash = {}",
            block.index, block.mining_time, block.hash
        );
    }

    println!("\nAverage mining time: {:.2} ms", blockchain.average_mining_time());

    blockchain.save_mining_stats("mining_stats.csv");
}