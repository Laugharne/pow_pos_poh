![](assets/pow_pos_poh.png)


# PoW ; PoS ; PoH

> **AVERTISSEMENT :** Pour tous les *"Jean-Michel-Premier-degré"*, les extraits de code dans cet article ne sont là qu'à titre d'illustration. Ce sont des versions **très simplifiées** des algorithmes de consensus, juste pour en illustrer le fonctionnement.


## TL;DR

1. Le mécanisme de consensus est un procédé par lequel les nœuds d'un réseau pair à pair se mettent d'accord sur un ensemble d'informations.
2. Le choix de l'algorithme de consensus a des implications sur la sécurité, la décentralisation, la consommation énergétique et divers aspects d'une blockchain.


## Introduction

Les algorithmes de **consensus** permettent de se mettre d'accord sur une **version commune et unique** des données partagées par une blockchain, et ce, malgré la possible présence de nœuds **défaillants** ou **malveillants**.

Les rôles des algorithmes de consensus sont les suivants :
1. **Accord sur l'état partagé :** Prouver que les transactions sont valides, leurs ordres, leurs origines.
2. **Résistance aux défaillances :** Être robuste. Le réseau doit fonctionner correctement même en présence de nœuds défaillants ou malveillants.
3. **Décentralisation :** Éviter la nécessité d'une autorité centrale. Cela permet une résistance accrue à la censure et à la centralisation d'une autorité.
4. **Sécurité :** Garantir l'intégrité des données, pas d'altération, préserver l'unicité.

La **synchronicité** et l'**unicité** en sont deux éléments très importants. En effet, savoir quand une transaction est arrivée en premier par rapport à une autre, de même que la garantie que les balances de comptes soient correctes sont essentielles, sinon il y a un risque de **double dépense**.

Il existe bien des manières de faire. En voici trois parmi les plus importantes ou usitées.
- La **preuve de travail** (*proof of work*) utilisée pour le **Bitcoin**.
- La **preuve d'enjeu** (*proof of stake*) utilisée maintenant par **Ethereum**.
- La **preuve d'historique** (*proof of history*) présente sur **Solana**.



## ⚒️ Proof of Work (PoW)

La preuve de travail est la première méthode de consensus utilisée dans **Bitcoin** basée sur l’algorithme **SHA-256** utilisé pour créer l’empreinte numérique d’un document. Pour chaque bloc, un nœud doit trouver une solution à un "*puzzle*" mathématique qui dépend du contenu du bloc et de son précédent, [comme illustré ici](https://andersbrownworth.com/blockchain/blockchain) (🇬🇧).

Notez qu’il n’y a pas de limite de nombre de participants car nul ne peut dire si quelqu’un va arriver en premier.

À cela s'ajoute la notion de **difficulté** de calcul qui consiste en un nombre variable et minimale de zéros à obtenir en début de résultat de hash (*leading zeros*) avec usage d'un *nonce* dans des itérations de calcul. Cette [difficulté est ajustée](https://www.blockchain.com/explorer/charts/difficulty) tous les **2016 blocs** (environs deux semaines) de manière à conserver un temps moyen entre chaque blocs en dessous de **10 minutes**.

Le premier nœud à résoudre correctement le calcul est récompensé par un certain nombre de bitcoins. Les nœuds vont essayer de trouver cette solution en utilisant leur puissance de calcul. Le temps nécessaire pour trouver la solution peut varier mais il y aura toujours un gagnant d’une quantité de Bitcoins.

Initialement, la récompense était de 50 bitcoins par bloc, mais cela est réduit de moitié environ tous les quatre ans dans un événement connu sous le nom de ["**halving**"](https://buybitcoinworldwide.com/halving/) (🇬🇧).

Au prochain halving (*article écrit début 2024*) qui aura lieu courant **2024**, la récompense passera de **6,25 BTC** à **3,125 BTC** par bloc.


![](assets/difficulty.png)

(*source : [buybitcoinworldwide.com](https://buybitcoinworldwide.com/halving/)*)


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

> Sur une courte séquence de **5 blocs**, ma simulation de calcul a dû procéder à **370894 calculs de hashs** avant de les valider tous.


## 💰 Proof of Stake (PoS)

La preuve d’enjeu est une alternative à la preuve de travail. Elle est utilisée par **Ethereum**. Contrairement à Bitcoin où les participants (*mineurs*) résolvent des problèmes complexes pour ajouter un bloc à la blockchain, la PoS requiert des efforts informatiques beaucoup moins intensifs.

Les participants (*validateurs*) sont choisis pour ajouter un nouveau bloc en fonction d'une quantité de cryptomonnaie qu'ils sont prêts à **"mettre en jeu"** **(staker)** en tant que garantie. Plus un participant en détient et est disposé à la bloquer, plus il a de chances d'être sélectionné pour créer un bloc. Les validateurs seront soit récompensés (*jetons, frais de transaction*) pour leur travail, soit [pénalisé en cas de malveillance](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/#pos-and-security) (🇬🇧).

Le choix des validateurs est déterminé par leur **enjeu** et il n'y a ainsi donc pas de nécessité à résoudre des problèmes mathématiques complexes. Par conséquent, la difficulté au sens de la recherche de *leading zeros* n'est pas applicable dans le contexte de ce consensus.

L'idée fondamentale étant que les individus/entités qui ont un **intérêt financier** dans la stabilité et la sécurité du réseau sont moins susceptibles de se comporter de manière malveillante. Leur participation au consensus est basée sur la possession d'une quantité de cryptomonnaie mise en jeu plutôt que sur la puissance de calcul.


**Version simplifiée de création de bloc (PoS) en Rust :**

```rust
fn creation_block(previous_block_hash: String, current_transactions: &Block) -> String {
    let to_hash: String    = format!("{}{}{}", previous_block_hash, current_transactions.index, current_transactions.data);
    let block_hash: String = digest(to_hash);
    block_hash
}
```

> Sur la même séquence de **5 blocs** que précédemment, il n'a fallu calculer que **5 hashs** pour les valider tous.


## 📜 Proof of History (PoH)

La preuve d'historique est utilisée par **Solana**. Reposant sur une base de données distribuée appelée *Account State*. Chaque transaction est stockée dans cette base de données. Pour qu'elles soient acceptées, elles doivent être liées à une **transaction précédente** existante. La validation d'une transaction précédente implique la validation de **toutes ses suivantes**.

En prenant un exemple simple, imaginez une **chaîne de montagnes** : pour en atteindre la fin, il faut d'abord gravir le premier sommet, puis le suivant, etc., jusqu'à atteindre le sommet final. La preuve d'historique garantit la validité de chaque transaction enchaînant la sienne à la précédente.

La PoS ajoute un registre d'historique des transactions et des blocs à chaque nœud. Cela permet aux utilisateurs de vérifier si leurs transactions ont été incluses dans le réseau ou pas.

En 2008, **Satoshi Nakamoto**, dans son **["White paper"](https://bitcoin.org/bitcoin.pdf)** (🇬🇧) a introduit le concept de "**timestamp server**". Bien qu'il n'utilise pas explicitement le terme "*blockchain*" dans ce document, il décrit les principes fondamentaux qui sous-tendent la technologie blockchain. Le "*timestamp server*" était un élément clé pour sécuriser l'ordre chronologique des transactions dans le système Bitcoin.

> Le terme "*blockchain*" par la suite, est devenu plus couramment utilisé pour décrire la structure de données décentralisée qui enregistre de manière immuable les transactions au travers de blocs connectés les uns aux autres à l'aide de fonctions de hachage cryptographiques.

Comme dit dans l'introduction, la synchronicité des... **BLABLABLA**

Par exemple chez Google, il est utilisé une horloge atomique afin de maintenir une unicité de temps entre tous ses services. Les blockchains n'utilisents pas ce genre de solution externe pour résoudre leur problème d'unicité de temps.

L'horodatage est directement encodé dans les messages de transaction.


(horodaté, associer une valeur temporelle à un évenement)

La chaîne de blocs peut être construite à partir d'un ensemble de transactions horodatée. Cela signifie que chaque message de transaction contient une information sur son temps et qu'il est possible de déterminer si un message a été ajouté avant ou après un autre message. Cela permet également de vérifier que toutes les transactions sont bien ordonnées chronologiquement.


Le PoH utilise une technique appelée "tick-counting" pour mesurer le temps. Chaque tick correspond à une petite quantité de temps réelle, mais il y a beaucoup plus de ticks par seconde que de secondes par tick. Les ticks sont utilisés pour incrémenter un compteur qui mesure le nombre de ticks passés depuis le début de l'univers. Ceci permet de générer une valeur unique pour chaque transaction, même s'ils ont lieu presque exactement au même moment.


Il est important de noter que le PoH ne garantit pas la chronologie absolue des transactions mais uniquement leur ordonnance relative. Cela signifie qu'une transaction peut arriver après une autre même si elle est antérieure.

Preuve d'ordonancement pourrait aussi être un terme utilisable pour la PoH.

...

Le PoH utilise une fonction `tick()` qui incrémente un compteur à chaque nouvelle transaction et ajoute cette valeur au hash du message de transaction. Cela permet de s'assurer que toutes les transactions sont ordonnées par rapport aux autres. La preuve d'historique est donc fournie par ce tick() qui est incorporé dans chaque message de transaction. On peut imaginer qu'il y ait un "ticker" central qui génère un nombre unique à chaque appel de `tick()`. Les utilisateurs peuvent alors ajouter ce numéro à leur message de transaction. Le nœud qui valide la transaction vérifie si le numéro est supérieur ou égal au précédent. Si c'est le cas, il accepte la transaction. Sinon, il rejette la transaction et attend jusqu'à ce que le ticker change.

...

**Version simplifiée de création de bloc (PoH) en Rust :**

```rust
// TODO
```



## Conclusions


--------

Crédits : **[Franck Maussand](mailto:franck@maussand.net)**

*Merci à [**Igor Bournazel**](https://github.com/ibourn) pour ses suggestions et la relecture de cet article.*

N'hésitez pas à jeter un coup d'oeil sur mon précédent article sur le [**function dispatcher des EVM**](https://medium.com/@franck.maussand/optimisation-sur-ethereum-faites-la-diff%C3%A9rence-avec-les-noms-de-fonctions-ba4692c9e39f) !

--------


## Ressources additionnelles

- **Hash :**
  - 🇫🇷 [Fonction de hachage — Wikipédia](https://fr.wikipedia.org/wiki/Fonction_de_hachage)
  - 🇬🇧 [Hash function - Wikipedia](https://en.wikipedia.org/wiki/Hash_function)
  - 🇫🇷 [SHA-3 — Wikipédia](https://fr.wikipedia.org/wiki/SHA-3)
  - 🇬🇧 [SHA-3 - Wikipedia](https://en.wikipedia.org/wiki/SHA-3)
  - 🇬🇧 [Blockchain Demo - Hash](https://andersbrownworth.com/blockchain/hash)


- **PoW :**
  - 🇫🇷 [Bitcoin : un système de paiement électronique pair-à-pair](https://bitcoin.org/files/bitcoin-paper/bitcoin_fr.pdf)
  - 🇬🇧 ["Bitcoin: A Peer-to-Peer Electronic Cash System"](https://bitcoin.org/bitcoin.pdf)
  - 🇬🇧 [Blockchain Demo - Blockchain](https://andersbrownworth.com/blockchain/blockchain)
  - 🇬🇧 [What is Proof of Work? (Cryptocurrency Explanation)](https://www.youtube.com/watch?v=XLcWy1uV8YM)
  - 🇬🇧 [Blockchain.com | Charts - Network Difficulty](https://www.blockchain.com/explorer/charts/difficulty)
  - 🇬🇧 [Next Bitcoin Halving 2024 Date & Countdown [BTC Clock]](https://buybitcoinworldwide.com/halving/)


- **PoS :**
  - 🇫🇷 [Proof of work / Proof of Stake : C'est quoi la différence ?](https://www.youtube.com/watch?v=dEGcAXeQsns)
  - 🇬🇧 [Proof-of-stake (PoS) | ethereum.org](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/)
  - 🇬🇧 [Proof-of-stake and security](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/#pos-and-security)
  - 🇬🇧 [What is Proof of Stake & How Does Confirmation Work in PoS?](https://coindcx.com/blog/crypto-basics/what-is-proof-of-stake-pos/)


- **PoH :**
  - 🇬🇧 [Solana: A new architecture for a high performance blockchain](https://solana.com/solana-whitepaper.pdf)
  - 🇬🇧 [Proof of History: A Clock for Blockchain](https://medium.com/solana-labs/proof-of-history-a-clock-for-blockchain-cf47a61a9274)
  - 🇬🇧 [Proof of History: How Solana brings time to crypto | Solana](https://solana.com/news/proof-of-history)
  - 🇬🇧 [Break | Solana](https://break.solana.com/)
  - 🇬🇧 [Proof of History Explained by a Water Clock](https://medium.com/solana-labs/proof-of-history-explained-by-a-water-clock-e682183417b8)


- **VDF :**
  - 🇬🇧 [Timelock Puzzles Using VDFs](https://medium.com/mistywest/timelock-puzzles-using-vdfs-b5636503950d)
  - 🇬🇧 [Day 54: VDFs: Verifiable Delay Functions in Blockchain](https://gsoares-block.medium.com/day-54-vdfs-verifiable-delay-functions-in-blockchain-addb3d89a72b)
  - 🇬🇧 [Verifiable Delay Functions](https://www.youtube.com/watch?v=_-feyaZZjEw)
  - 🇬🇧 [Verifiable Delay Functions: Applications and Candidate Constructions - BPASE '18](https://www.youtube.com/watch?v=qUoagL7OZ1k)


