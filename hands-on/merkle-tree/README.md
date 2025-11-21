## Merkle Tree

The implementation of Merkle Tree in Rust.

This tree is stored in a vector. 

For example, there are four items, merkle tree is kept like: 
[hash0,hash1,hash2,hash3,hash01,hash23,root]

While building a tree, if there is an odd number of nodes at the given level, the last node will be duplicated.

### Run unit tests

```bash
cargo test
```

### Usage example

```Rust
use ring::digest::{Algorithm, SHA512};
use merkle_tree::MerkleTree;

static ALGO: &'static Algorithm = &SHA512;

fn main() {
    let values = vec!["one", "two", "three", "four"];
    let tree = MerkleTree::new(&values, ALGO);
    let proof = tree.build_proof(&"one");
    let vec = proof.unwrap();
    tree.validate(&vec);
}
```
