use chrono::prelude::*;
use sha2::{Sha256, Digest};
use hex;

#[derive(Serialize, Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    prev_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct Chain {
    chain: Vec<Block>,
    pending: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        Chain {
            chain: Vec::new(),
            pending: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        }
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.pending.push(Transaction { sender, receiver, amount });
        true
    }

    pub fn mine(&mut self) -> bool {
        let header = Blockheader {
            timestamp: Utc::now().timestamp_millis(),
            nonce: 0,
            prev_hash: self.last_block_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_tx = Transaction {
            sender: "Root".to_string(),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_tx);
        block.transactions.append(&mut self.pending);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Chain::merkle(&block.transactions);
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);

        true
    }

    /// This method will update the nonce value in header
    pub(crate) fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            // Take first {difficulty} chars of hash then parse resulted string to u32
            //  and check if parsed is equals to 0.
            // Which means that first {difficulty} chars of string are zeros
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("block hash/nonce: {}/{}", hash, header.nonce);
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                }
            }
        }
    }

    fn merkle(transactions: &[Transaction]) -> String {
        let mut merkle = Vec::new();

        for t in transactions {
            merkle.push(Chain::hash(t));
        }

        if merkle.len() % 2 == 1 {
            merkle.push(merkle.last().unwrap().clone());
        }

        while merkle.len() > 1 {
            let mut hash_1 = merkle.remove(0);
            let hash_2 = merkle.remove(0);
            hash_1.push_str(&hash_2);
            let new_hash = Chain::hash(&hash_1);
            merkle.push(new_hash);
        }

        merkle.pop().unwrap()
    }

    pub(crate) fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result().to_vec();

        hex::encode(res)
    }

    pub (crate) fn last_block_hash(&self) -> String {
        if let Some(block) = self.chain.last() {
            return Chain::hash(&block.header);
        }
        String::from_utf8(vec![48; 64]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::Chain;

    #[test]
    fn test_new_transaction_added_to_pending() {
        let mut chain = Chain::new("vasya".to_string(), 4);
        chain.new_transaction("vasya".to_string(), "marina".to_string(), 98.0);
        assert_eq!(chain.pending.len(), 1);
    }

    #[test]
    fn test_proof_of_work() {
        let mut chain = Chain::new("vasya".to_string(), 3);
        chain.new_transaction("vasya".to_string(), "marina".to_string(), 98.0);
        chain.mine();
        let last_block = chain.chain.pop().unwrap();
        let header = last_block.header;
        let hashed = Chain::hash(&header);
        let hash_first_symbols = &hashed[..header.difficulty as usize];
        assert_eq!("000", hash_first_symbols)
    }

    #[test]
    fn test_prev_hash() {
        let mut chain = Chain::new("vasya".to_string(), 3);
        chain.mine();
        chain.new_transaction("vasya".to_string(), "marina".to_string(), 98.0);
        chain.mine();
        let last_block = chain.chain.pop().unwrap();
        let last_block_hash = chain.last_block_hash();
        assert_eq!(last_block_hash, last_block.header.prev_hash);
    }
}
