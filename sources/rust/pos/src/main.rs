use sha256::digest;

#[derive(Debug)]
struct Block {
	index: u32,
	data : String,
}


fn validation_block(previous_block_hash: String, current_transactions: &Block) -> String {
	let to_hash: String    = format!("{}{}{}", previous_block_hash, current_transactions.index, current_transactions.data);
	let block_hash: String = digest(to_hash);
	block_hash
}


fn set_block(index: u32, transactions: String) -> Block {
	Block {
		index: index,
		data : transactions,
	}	
}


fn main() {
	println!("Hello, proof of stack!");

	let mut block_chain: Vec<Block> = Vec::new();

	block_chain.push(set_block(0, "First transaction from Genesis block.".to_string()));
	block_chain.push(set_block(1, "Alyra".to_string()));
	block_chain.push(set_block(2, "the".to_string()));
	block_chain.push(set_block(3, "blockchain".to_string()));
	block_chain.push(set_block(4, "school!".to_string()));

	let mut previous_hash = "00".repeat(32);

	block_chain.iter().enumerate().for_each(|(_,block)| {
		let hash = validation_block(
			previous_hash.clone(),
			block,
		);

		previous_hash = hash;

	});
	// for block in &mut block_chain {
	// 	let hash = compute_block(
	// 		previous_hash.clone(),
	// 		block
	// 	);

	// 	previous_hash = hash;
	// }

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
	fn validation_block_genesis() {

		let genesis_block: Block = Block {
			index: 0,
			data : "First transaction from Genesis block.".to_string(),
		};
	
		let hash = validation_block(
			"00".repeat(32).to_string(),
			&genesis_block
		);
		assert_eq!(hash, "610b5b47edd3c32f80f4ebd9f5f9a1e1b5ee570a1e5949aa7af0cf5741cb969f");

	}

	#[test]
	fn validation_block_alyra() {

		let alyra_block: Block = Block {
			index: 1,
			data : "Alyra".to_string(),
		};
	
		let hash = validation_block(
			"610b5b47edd3c32f80f4ebd9f5f9a1e1b5ee570a1e5949aa7af0cf5741cb969f".to_string(),
			&alyra_block
		);
		assert_eq!(hash, "2cb1d211d3fc16f4037cf287eef4248b4c7a51c34caae2dc2e386a26c8952fe9");

	}

}