use sha256::digest;

#[derive(Debug)]
struct Block {
	index: u32,
	nonce: u32,
	data : String,
	hash : String,
}



fn mining(previous_block_hash: String, current_transactions: &mut Block, difficulty: usize) -> (String, u32) {
	let prefix: String = "00".repeat(difficulty);
	let mut nonce: u32 = 0;

	loop {
		let to_hash: String    = format!("{}{}{}{}", previous_block_hash, current_transactions.index, current_transactions.data, nonce);
		let block_hash: String = digest(to_hash);
		
		if block_hash.starts_with(&prefix) {
			return (block_hash, nonce);
		} else {
			nonce += 1;
		}
	}
}


fn set_block(index: u32, transactions: String) -> Block {
	Block {
		index     : index,
		nonce     : 0,
		data      : transactions,
		hash      : "".to_string(),
	}
}


fn main() {
	println!("Hello, proof of work!");

	let mut block_chain: Vec<Block> = Vec::new();

	block_chain.push(set_block(0, "First transaction from Genesis block.".to_string()));
	block_chain.push(set_block(1, "Alyra".to_string()));
	block_chain.push(set_block(2, "the".to_string()));
	block_chain.push(set_block(3, "blockchain".to_string()));
	block_chain.push(set_block(4, "school!".to_string()));

	let mut previous_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

	for block in &mut block_chain {
		let (hash, nonce) = mining(
			previous_hash.clone(),
			block,
			2
		);

		block.nonce   = nonce;
		block.hash    = hash.clone();
		previous_hash = hash;
	}

	dbg!(block_chain);

}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sha256_checks() {
		let input: String = String::from("hello");
		let val: String   = digest(input);
		assert_eq!(val,"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
		
		//sha256 digest &str
		let input: &str = "hello";
		let val: String = digest(input);
		assert_eq!(val,"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
		
		//sha256 digest &mut &str
		let mut input: &str = "hello";
		let val: String     = digest(&mut input);
		assert_eq!(val,"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
		
		//sha256 digest char
		let input: &str = "π";
		let val: String = digest(input);
		assert_eq!(val,"2617fcb92baa83a96341de050f07a3186657090881eae6b833f66a035600f35a");
	
	
		let input: &[u8; 5] = b"hello";
		let val: String     = digest(input);
		assert_eq!(val, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
	}


	#[test]
	fn mining_genesis() {
		let mut genesis_block: Block = Block {
			index: 0,
			nonce: 42,
			data : "First transaction from Genesis block.".to_string(),
			hash : "".to_string(),
		};
	
		let (hash, nonce) = mining(
			"0000000000000000000000000000000000000000000000000000000000000000".to_string(),
			&mut genesis_block,
			2
		);
		assert_eq!(hash, "0000a0d03a84ce3be1458b7df3586876dcee8caa1aa518e27dd8a086a1de30b0");
		assert_eq!(nonce, 1971);
	}

	#[test]
	fn mining_block() {
		let mut alyra_block: Block = Block {
			index: 1,
			nonce: 42,
			data : "Alyra".to_string(),
			hash : "".to_string(),
		};
	
		let (hash, nonce) = mining(
			"0000a0d03a84ce3be1458b7df3586876dcee8caa1aa518e27dd8a086a1de30b0".to_string(),
			&mut alyra_block,
			2
		);
		assert_eq!(hash, "0000f8bde4bf5fc9597721996524a1ca6c32635661ad3cf79a397b4177e1ac15");
		assert_eq!(nonce, 2659);
	}

}