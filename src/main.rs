use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender_key: String,
    receiver_key: String,
    amount: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(
        index: u32,
        timestamp: u128,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.mine_block();
        block
    }

    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn mine_block(&mut self) -> String {
        loop {
            let hash = self.calculate_hash();
            if &hash[..4] == "0000" {
                return hash;
            }
            self.nonce += 1;
        }
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    transaction: Vec<Transaction>,
    amount: u32,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
            transaction: Vec::new(),
            amount: 0,
        }
    }

    fn create_genesis_block() -> Block {
        Block::new(0, current_timestamp(), Vec::new(), String::from("0"))
    }

    fn create_transaction(&mut self, transaction: Transaction) {
        let amount = transaction.amount;
        self.transaction.push(transaction);
        self.amount += amount;
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

fn current_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.create_transaction(Transaction {
        sender_key: String::from("Alice"),
        receiver_key: String::from("Bob"),
        amount: 100,
    });

    blockchain.create_transaction(Transaction {
        sender_key: String::from("Bob"),
        receiver_key: String::from("Alice"),
        amount: 50,
    });

    println!("Blockchain: {:#?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
}
