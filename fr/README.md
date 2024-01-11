![](assets/pow_pos_poh.png)


# PoW ; PoS ; PoH

> **AVERTISSEMENT** : Pour tous les *"Jean-Michel-Premier-degré"*, les extraits de code dans cet article ne sont là qu'à titre d'illustration. Ce sont des versions très simplifiées des algorithmes de consensus pour en illustrer le fonctionnement.


## TL;DR


## Introduction

Le mécanisme de **consensus**, est un procédé par lequel les noeuds d'un réseau pair à pair se mettent d'accord sur un ensemble d'informations, et permet ainsi de se mettre d'accord sur une **version commune et unique** des données partagées, malgré la possible présence de nœuds **défaillants** ou **malveillants**.

Les rôles des algorithmes de consensus dans les blockchains sont les suivants :
1. **Accord sur l'état partagé :** Prouver que les transactions sont valides, leurs ordres, leurs origines.
2. **Résistance aux défaillances :** Le réseau de fonctionner correctement même en présence de nœuds défaillants, malveillants.
3. **Décentralisation :** Évitant ainsi la nécessité d'une autorité centrale. Cela permet une résistance accrue à la censure et à la centralisation.
4. **Sécurité :** Garantir l'intégrité des données, pas d'altération, préserver l'unicité.

En effet, il est **très important** de savoir quand une transaction est arrivée en premier par rapport à une autre, sinon il y a le risque de **double dépense**.

Il existe plusieurs manière de faire...


## ⚒️ Proof of Work (PoW)

**Version simplifiée du minage (PoW) en Rust :**
```rust
fn mining_block(previous_block_hash: String, current_transactions: &Block, difficulty: usize) -> (String, u32) {
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
```

## 💰 Proof of Stake (PoS)


**Version simplifiée de création de bloc (PoW) en Rust :**
```rust
fn creation_block(previous_block_hash: String, current_transactions: &Block) -> String {
	let to_hash: String    = format!("{}{}{}", previous_block_hash, current_transactions.index, current_transactions.data);
	let block_hash: String = digest(to_hash);
	block_hash
}
```


## 📜 Proof of History (PoH)

En 2008, **Satoshi Nakamoto**, dans son ["White paper"](https://bitcoin.org/bitcoin.pdf) (🇬🇧) a introduit le concept de "**timestamp server**". Bien qu'il n'utilise pas explicitement le terme "*blockchain*" dans ce document, il décrit les principes fondamentaux qui sous-tendent la technologie blockchain. Le "*timestamp server*" était un élément clé pour sécuriser l'ordre chronologique des transactions dans le système Bitcoin.

Le terme "*blockchain*" par la suite, est devenu plus couramment utilisé pour décrire la structure de données décentralisée qui enregistre de manière immuable les transactions au travers de blocs connectés les uns aux autres à l'aide de fonctions de hachage cryptographiques.



## Conclusions


--------

Crédits : **[Franck Maussand](mailto:franck@maussand.net)**

*Merci à [**Igor Bournazel**](https://github.com/ibourn) pour ses suggestions et la relecture de cet article.*

N'hésitez pas à jeter un coup d'oeiul sur mon précédent article sur le [**function dispatcher des EVM**](https://medium.com/@franck.maussand/optimisation-sur-ethereum-faites-la-diff%C3%A9rence-avec-les-noms-de-fonctions-ba4692c9e39f) !

--------


## Ressources additionnelles

- **Hash :**
  - 🇫🇷 [Fonction de hachage — Wikipédia](https://fr.wikipedia.org/wiki/Fonction_de_hachage)
  - 🇬🇧 [Hash function - Wikipedia](https://en.wikipedia.org/wiki/Hash_function)
  - 🇫🇷 [SHA-3 — Wikipédia](https://fr.wikipedia.org/wiki/SHA-3)
  - 🇬🇧 [SHA-3 - Wikipedia](https://en.wikipedia.org/wiki/SHA-3)
  - 🇬🇧 [Blockchain Demo](https://andersbrownworth.com/blockchain/hash)

- **PoW :**
  - 🇫🇷 [Bitcoin : un système de paiement électronique pair-à-pair](https://bitcoin.org/files/bitcoin-paper/bitcoin_fr.pdf)
  - 🇬🇧 ["Bitcoin: A Peer-to-Peer Electronic Cash System"](https://bitcoin.org/bitcoin.pdf)
  - 🇬🇧 [Blockchain Demo](https://andersbrownworth.com/blockchain/blockchain)
  - 🇬🇧 [What is Proof of Work? (Cryptocurrency Explanation) - YouTube](https://www.youtube.com/watch?v=XLcWy1uV8YM)

- **PoS :**
  - 🇫🇷 [Proof of work / Proof of Stake : C'est quoi la différence ? - YouTube](https://www.youtube.com/watch?v=dEGcAXeQsns)
  - 🇬🇧 [Proof of History: A Clock for Blockchain](https://medium.com/solana-labs/proof-of-history-a-clock-for-blockchain-cf47a61a9274)
  - 🇬🇧 [What is Proof of Stake & How Does Confirmation Work in PoS?](https://coindcx.com/blog/crypto-basics/what-is-proof-of-stake-pos/)

- **PoH :**
  - 🇬🇧 [Solana: A new architecture for a high performance blockchain](https://solana.com/solana-whitepaper.pdf)
  - 🇬🇧 [Proof of History: How Solana brings time to crypto | Solana](https://solana.com/news/proof-of-history)
  - 🇬🇧 [Break | Solana](https://break.solana.com/)
  - 🇬🇧 [Proof of History Explained by a Water Clock](https://medium.com/solana-labs/proof-of-history-explained-by-a-water-clock-e682183417b8)

- **VDF :**
  - 🇬🇧 [Verifiable Delay Functions - YouTube](https://www.youtube.com/watch?v=_-feyaZZjEw)
  - 🇬🇧 [Timelock Puzzles Using VDFs](https://medium.com/mistywest/timelock-puzzles-using-vdfs-b5636503950d)
  - 🇬🇧 [Day 54: VDFs: Verifiable Delay Functions in Blockchain](https://gsoares-block.medium.com/day-54-vdfs-verifiable-delay-functions-in-blockchain-addb3d89a72b)


