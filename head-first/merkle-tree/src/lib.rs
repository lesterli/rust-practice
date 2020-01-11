/// The implementation of Merkle Tree in Rust.

extern crate ring;

use std::convert::AsRef;
use std::hash::Hash;

use ring::digest::{Algorithm, Context, Digest};

/// This tree is stored in a vector.
///
/// For example, there are four items, merkle tree is kept like:
/// [hash0,hash1,hash2,hash3,hash01,hash23,root]
///
/// # Usage example
///
/// ```
/// use ring::digest::{Algorithm, Context, Digest, SHA512};
/// use merkle_tree::MerkleTree;
///
/// static ALGO: &'static Algorithm = &SHA512;
///
/// let values = vec!["one", "two", "three", "four"];
/// let tree = MerkleTree::new(&values, ALGO);
/// let proof = tree.build_proof(&"one");
/// let vec = proof.unwrap();
/// tree.validate(&vec);
/// ```
///

pub struct MerkleTree {
    array: Vec<u8>,
    height: usize,
    items_count: usize,
    algo: &'static Algorithm,
}

impl MerkleTree {
    /// Build Merkle Tree
    pub fn new<T: AsRef<[u8]>>(values: &Vec<T>, algo: &'static Algorithm) -> MerkleTree {
        let (height, array) = build_tree(values, algo);
        MerkleTree {
            array: array,
            height: height,
            items_count: values.len(),
            algo: algo,
        }
    }

    /// Generate Merkle Proof
    pub fn build_proof<T: Eq + Hash + AsRef<[u8]>>(&self, value: &T) -> Option<Vec<&[u8]>> {
        let hash = get_hash(value.as_ref(), self.algo).as_ref().to_vec();
        let index = self.find_item(&hash);
        let mut vec = vec![];
        match index {
            Some(i) => {
                vec.push(&self.array[(i * self.algo.output_len)..(i * self.algo.output_len + self.algo.output_len)]);
                Some(self.add_level(0, i, self.items_count, vec))
            }
            None => None
        }
    }

    fn find_item(&self, hash: &Vec<u8>) -> Option<usize> {
        let mut result = None;
        // linear search item in a loop
        for index in 0..self.items_count {
            let start = index * self.algo.output_len;
            if hash.as_slice() == &self.array[start..(start + self.algo.output_len)] {
                result = Some(index);
                break;
            }
        }
        result
    }

    /// Recursion
    fn add_level<'a>(&'a self, start_index: usize, index: usize, mut level_len: usize, mut result: Vec<&'a [u8]>) -> Vec<&'a [u8]> {
        level_len += level_len & 1;
        let (sibling, parent) = calculate_relatives(index);
        //Add sibling to result
        result.push(&self.array[
            (start_index + sibling * self.algo.output_len)..(start_index + sibling * self.algo.output_len + self.algo.output_len)
            ]);
        let next_level_len = level_len / 2;
        // Do not include root to proof
        if next_level_len == 1 { 
            return result;
        }
        self.add_level(start_index + level_len * self.algo.output_len, parent, next_level_len, result)
    }

    pub fn is_empty(&self) -> bool {
        self.nodes_count() == 0
    }

    pub fn get_root(&self) -> &[u8] {
        if self.is_empty() {
            return &[];
        }
        let root_index = self.array.len() - self.algo.output_len;
        &self.array[root_index..] // Last item
    }

    pub fn nodes_count(&self) -> usize {
        self.array.len() / self.algo.output_len
    }

    pub fn leafs_count(&self) -> usize {
        self.items_count
    }

    pub fn data_size(&self) -> usize {
        self.array.len()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// fold() takes two arguments: an initial hash(01)
    /// and a closure with two arguments 
    pub fn validate(&self, proof: &Vec<&[u8]>) -> bool {
        proof[2..].iter()
            .fold(
                get_pair_hash(proof[0], proof[1], self.algo),
                |a, b| get_pair_hash(a.as_ref(), b, self.algo)
            ).as_ref() == self.get_root()
    }
}

/// "2i 2i+1" schema
fn calculate_relatives(index: usize) -> (usize, usize) {
    let mut sibling = index;
    if index & 1 == 0 {
        sibling += 1
    } else {
        sibling -= 1
    };
    let parent = (index + 1 + ((index + 1) & 1)) / 2 - 1;
    (sibling, parent)
}

/// While building a tree, if there is an odd number of nodes at the given
/// level, the last node will be duplicated.
fn build_tree<T: AsRef<[u8]>>(values: &Vec<T>, algo: &'static Algorithm) -> (usize, Vec<u8>) {
    let vec_len = calculate_vec_len(values.len(), algo);
    let mut tree: Vec<u8> = Vec::with_capacity(vec_len);
    for (_i, v) in values.iter().enumerate() { //Hash leafs
        let digest = get_hash(v.as_ref(), algo);
        let hash = digest.as_ref();
        tree.extend_from_slice(hash);
    }
    let height = build_level(&mut tree, 0, values.len(), algo);
    (height, tree)
}

/// length = (leafs + nodes) * output_len
fn calculate_vec_len(len: usize, algo: &'static Algorithm) -> usize {
    //Determine leafs number is even or odd  
    let mut result = len + (len & 1);
    let mut level = result;
    while level > 1 {
        level += level & 1;
        level = level / 2;
        result += level;
    }
    //output_len is the length of a finalized digest
    result * algo.output_len
}

/// Return tree weight and build nodes
fn build_level(tree: &mut Vec<u8>, prev_level_start: usize, mut prev_level_len: usize, algo: &'static Algorithm) -> usize {
    if prev_level_len & 1 == 1 {
        //Previous level has odd number of children
        let prev = &tree[(prev_level_start * algo.output_len + (prev_level_len - 1) * algo.output_len)..]
            .to_owned();
        //Duplicate last item
        tree.extend_from_slice(prev); 
        prev_level_len += 1;
    }
    let level_len = prev_level_len / 2;
    for i in 0..level_len {
        let begin = prev_level_start * algo.output_len + i * 2 * algo.output_len;
        let middle = begin + algo.output_len;
        let end = middle + algo.output_len;
        let hash = get_pair_hash(
            &tree[begin..middle], //Left node
            &tree[middle..end], //Right node
            algo);
        tree.extend_from_slice(hash.as_ref());
    };
    if level_len > 1 {
        return build_level(tree, prev_level_start + prev_level_len, level_len, algo) + 1;
    }
    if level_len > 0 {
        return 2;
    }
    return 0;
}

/// Generate Node hash
pub fn get_pair_hash(x: &[u8], y: &[u8], algo: &'static Algorithm) -> Digest {
    let left = x;
    let right = y;
    let mut ctx = Context::new(algo);
    ctx.update(left);
    ctx.update(right);
    ctx.finish()
}

/// Hash function
pub fn get_hash(x: &[u8], algo: &'static Algorithm) -> Digest {
    let mut ctx = Context::new(algo);
    ctx.update(x);
    ctx.finish()
}

#[cfg(test)]
mod tests {
    use ring::digest::{Algorithm, Context, Digest, SHA512};
    use super::MerkleTree;

    static ALGO: &'static Algorithm = &SHA512;

    #[test]
    fn test_build_tree_with_0_values() {
        let values: Vec<&str> = vec![];
        let tree = MerkleTree::new(&values, ALGO);

        assert_eq!(true, tree.is_empty());
        assert_eq!(0, tree.height());
        assert_eq!(0, tree.nodes_count());
        assert_eq!(0, tree.data_size());
        let empty_root: Vec<u8> = vec![];
        assert_eq!(empty_root, tree.get_root());
    }

    #[test]
    fn test_build_tree_with_odd_number_of_values() {
        let values = vec!["one", "two", "three"];
        let tree = MerkleTree::new(&values, ALGO);

        let _d0: Digest = super::get_hash(values[0].as_ref(), ALGO);
        let _d1: Digest = super::get_hash(values[1].as_ref(), ALGO);
        let _d2: Digest = super::get_hash(values[2].as_ref(), ALGO);
        let _d3: Digest = super::get_hash(values[2].as_ref(), ALGO);

        let _d01 = hash_pair(_d0.as_ref(), _d1.as_ref(), ALGO);
        let _d23 = hash_pair(_d2.as_ref(), _d3.as_ref(), ALGO);
        let _pair = super::get_pair_hash(_d01.as_ref(), _d23.as_ref(), ALGO);

        assert_eq!(false, tree.is_empty());
        assert_eq!(3, tree.height());
        assert_eq!(7, tree.nodes_count());
        assert_eq!(7 * ALGO.output_len, tree.data_size());
        assert_eq!(_pair.as_ref(), tree.get_root());
    }

    #[test]
    fn test_build_tree_with_even_number_of_values() {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::new(&values, ALGO);

        let _d0: Digest = super::get_hash(values[0].as_ref(), ALGO);
        let _d1: Digest = super::get_hash(values[1].as_ref(), ALGO);
        let _d2: Digest = super::get_hash(values[2].as_ref(), ALGO);
        let _d3: Digest = super::get_hash(values[3].as_ref(), ALGO);

        let _d01 = hash_pair(_d0.as_ref(), _d1.as_ref(), ALGO);
        let _d23 = hash_pair(_d2.as_ref(), _d3.as_ref(), ALGO);
        let _pair = super::get_pair_hash(_d01.as_ref(), _d23.as_ref(), ALGO);

        assert_eq!(false, tree.is_empty());
        assert_eq!(3, tree.height());
        assert_eq!(7, tree.nodes_count());
        assert_eq!(7 * ALGO.output_len, tree.data_size());
        assert_eq!(_pair.as_ref(), tree.get_root());
    }

    #[test]
    fn test_root_hash_same_if_values_were_same() {
        let values = vec!["one", "one", "one", "one"];
        let tree = MerkleTree::new(&values, ALGO);

        let _d0: Digest = super::get_hash(values[0].as_ref(), ALGO);
        let _d1: Digest = super::get_hash(values[1].as_ref(), ALGO);
        let _d2: Digest = super::get_hash(values[2].as_ref(), ALGO);
        let _d3: Digest = super::get_hash(values[3].as_ref(), ALGO);

        let _d01 = hash_pair(_d0.as_ref(), _d1.as_ref(), ALGO);
        let _d23 = hash_pair(_d2.as_ref(), _d3.as_ref(), ALGO);
        let _pair = super::get_pair_hash(_d23.as_ref(), _d01.as_ref(), ALGO);

        assert_eq!(false, tree.is_empty());
        assert_eq!(3, tree.height());
        assert_eq!(7, tree.nodes_count());
        assert_eq!(7 * ALGO.output_len, tree.data_size());
        assert_eq!(_pair.as_ref(), tree.get_root());
    }

    #[test]
    fn test_root_hash_different_reverse_values() {
        let values1 = vec!["one", "two"];
        let tree1 = MerkleTree::new(&values1, ALGO);

        let values2 = vec!["two", "one"];
        let tree2 = MerkleTree::new(&values2, ALGO);

        assert_ne!(tree1.get_root(), tree2.get_root());
    }

    #[test]
    fn test_generate_merkle_proof_and_validate() {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::new(&values, ALGO);

        for v in values {
            let proof = tree.build_proof(&v);
            assert_eq!(true, proof.is_some());
            let vec = proof.unwrap();
            assert_eq!(3, vec.len());
            tree.validate(&vec);
        }

        let absent = vec!["qqq", "www", "eee", "rrr"];
        for v in absent {
            let proof = tree.build_proof(&v);
            assert_eq!(true, proof.is_none());
        }
    }

    #[test]
    fn test_provide_bad_merkle_proof() {
        let values = vec!["one", "two", "three", "four"];
        let tree = MerkleTree::new(&values, ALGO);
        let proof = tree.build_proof(&"one");

        assert_eq!(true, proof.is_some());
        let _d0: Digest = super::get_hash("five".as_ref(), ALGO);
        let proof_vec = proof.unwrap();
        let vec = vec![proof_vec[0], proof_vec[1], _d0.as_ref()];
        assert_eq!(false, tree.validate(&vec));
    }

    // helper function
    fn hash_pair(x: &[u8], y: &[u8], algo: &'static Algorithm) -> Digest {
        let mut ctx = Context::new(algo);
        ctx.update(x);
        ctx.update(y);
        ctx.finish()
    }
    
}



