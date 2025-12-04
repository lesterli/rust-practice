use crate::models::*;
use blake3::Hasher;

/// Semantic hashing utilities for deduplication
///
/// This module provides functions to generate strong semantic hashes
/// from normalized Rust code items, ensuring that functionally equivalent
/// code produces the same hash regardless of formatting differences.

/// Generate a semantic hash from an ExtractedItem's normalized content
///
/// This function creates a BLAKE3 hash of the normalized code body,
/// signature, and other semantic content to create a unique identifier
/// for deduplication purposes.
pub fn semantic_hash_item(item: &ExtractedItem) -> CoreResult<String> {
    // Create a hasher for this item
    let mut hasher = Hasher::new();

    // Hash the normalized body (most important for semantic similarity)
    hasher.update(item.content.body_normalized.as_bytes());

    // Hash the signature
    hasher.update(item.content.signature.as_bytes());

    // Hash the item kind
    hasher.update(item.item_meta.kind.to_string().as_bytes());

    // Generate the final hash
    let hash = hasher.finalize();

    // Return as hex string with blake3: prefix
    Ok(format!("blake3:{}", hash.to_hex()))
}

/// Generate a semantic hash from just the normalized code string
///
/// This is useful for comparing code similarity without full metadata.
pub fn semantic_hash_code(code: &str, item_kind: &str) -> CoreResult<String> {
    let mut hasher = Hasher::new();

    hasher.update(code.as_bytes());
    hasher.update(item_kind.as_bytes());

    let hash = hasher.finalize();
    Ok(format!("blake3:{}", hash.to_hex()))
}

/// Batch hash multiple items efficiently
///
/// This function takes a slice of items and returns a vector of their semantic hashes.
pub fn semantic_hash_items(items: &[ExtractedItem]) -> CoreResult<Vec<String>> {
    let mut hashes = Vec::with_capacity(items.len());

    for item in items {
        hashes.push(semantic_hash_item(item)?);
    }

    Ok(hashes)
}

/// Update an ExtractedItem with its semantic hash
///
/// This is a convenience function that both generates and assigns the hash
/// to the item's content.
pub fn hash_and_update_item(item: &mut ExtractedItem) -> CoreResult<()> {
    let hash = semantic_hash_item(item)?;
    item.content.semantic_hash = hash;
    Ok(())
}

/// Batch hash and update multiple items
///
/// This function efficiently processes multiple items, generating their
/// semantic hashes and updating their content in place.
pub fn hash_and_update_items(items: &mut [ExtractedItem]) -> CoreResult<()> {
    for item in items {
        hash_and_update_item(item)?;
    }
    Ok(())
}

/// Verify semantic similarity between two items
///
/// Returns true if the two items have identical semantic content
/// (normalized body, signature, and type).
pub fn are_semantically_similar(item1: &ExtractedItem, item2: &ExtractedItem) -> CoreResult<bool> {
    let hash1 = semantic_hash_item(item1)?;
    let hash2 = semantic_hash_item(item2)?;

    Ok(hash1 == hash2)
}

/// Extract just the hash bytes for low-level comparisons
///
/// Returns the raw BLAKE3 hash bytes, useful for custom similarity
/// algorithms or cryptographic operations.
pub fn hash_bytes(item: &ExtractedItem) -> CoreResult<[u8; 32]> {
    let mut hasher = Hasher::new();

    hasher.update(item.content.body_normalized.as_bytes());
    hasher.update(item.content.signature.as_bytes());
    hasher.update(item.item_meta.kind.to_string().as_bytes());

    let hash = hasher.finalize();
    Ok(*hash.as_bytes())
}

/// Calculate Hamming distance between two semantic hashes
///
/// Returns the number of differing bits between two hash values.
/// Useful for approximate similarity matching.
pub fn hash_hamming_distance(hash1: &str, hash2: &str) -> Option<usize> {
    if !hash1.starts_with("blake3:") || !hash2.starts_with("blake3:") {
        return None;
    }

    let bytes1 = &hex::decode(&hash1[7..]).ok()?[0..32];
    let bytes2 = &hex::decode(&hash2[7..]).ok()?[0..32];

    let mut distance = 0;
    for i in 0..32 {
        let xored = bytes1[i] ^ bytes2[i];
        // Count set bits in the xor result
        distance += xored.count_ones() as usize;
    }

    Some(distance)
}

/// Find similar items within a collection
///
/// Given a collection of items and a similarity threshold (0-255),
/// returns pairs of indices that are semantically similar.
/// Lower threshold means more similarity required.
pub fn find_similar_items(
    items: &[ExtractedItem],
    max_distance: u8,
) -> CoreResult<Vec<(usize, usize, usize)>> {
    let mut similar_pairs = Vec::new();
    let hashes: Vec<String> = semantic_hash_items(items)?;

    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if let Some(distance) = hash_hamming_distance(&hashes[i], &hashes[j]) {
                if distance <= max_distance as usize {
                    similar_pairs.push((i, j, distance));
                }
            }
        }
    }

    Ok(similar_pairs)
}

/// Build a deduplication map from semantic hashes
///
/// Returns a HashMap where keys are semantic hashes and values are
/// indices of items with that hash. Useful for removing duplicates.
pub fn build_dedup_map(
    items: &[ExtractedItem],
) -> CoreResult<std::collections::HashMap<String, Vec<usize>>> {
    let mut dedup_map = std::collections::HashMap::new();
    let hashes = semantic_hash_items(items)?;

    for (idx, hash) in hashes.into_iter().enumerate() {
        dedup_map.entry(hash).or_insert_with(Vec::new).push(idx);
    }

    Ok(dedup_map)
}

/// Statistics about hash distribution
///
/// Provides insights into the quality of semantic hashing by analyzing
/// hash collisions and distribution.
pub struct HashStats {
    pub total_items: usize,
    pub unique_hashes: usize,
    pub collision_count: usize,
    pub max_collision_group: usize,
    pub collision_rate: f64,
}

impl HashStats {
    /// Analyze hash statistics for a collection of items
    pub fn analyze(items: &[ExtractedItem]) -> CoreResult<Self> {
        let dedup_map = build_dedup_map(items)?;
        let unique_hashes = dedup_map.len();
        let mut collision_count = 0;
        let mut max_collision_group = 0;

        for group in dedup_map.values() {
            if group.len() > 1 {
                collision_count += group.len() - 1;
            }
            max_collision_group = max_collision_group.max(group.len());
        }

        let collision_rate = if items.is_empty() {
            0.0
        } else {
            collision_count as f64 / items.len() as f64
        };

        Ok(HashStats {
            total_items: items.len(),
            unique_hashes,
            collision_count,
            max_collision_group,
            collision_rate,
        })
    }

    /// Pretty-print the statistics
    pub fn print(&self) {
        println!("Hash Statistics:");
        println!("  Total items: {}", self.total_items);
        println!("  Unique hashes: {}", self.unique_hashes);
        println!("  Collisions: {}", self.collision_count);
        println!("  Max collision group: {}", self.max_collision_group);
        println!("  Collision rate: {:.2}%", self.collision_rate * 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_hashing_consistency() {
        use crate::models::*;

        let item1 = ExtractedItem {
            project_context: ProjectContext {
                repo_url: "test".to_string(),
                commit_hash: "test".to_string(),
                file_path: "test.rs".to_string(),
            },
            item_meta: ItemMeta {
                kind: ItemKind::Function,
                name: "test_fn".to_string(),
                fully_qualified_name: "test::test_fn".to_string(),
                start_line: 1,
                end_line: 5,
            },
            content: Content {
                signature: "fn test()".to_string(),
                body_normalized: "fn test() {}".to_string(),
                semantic_hash: String::new(),
                docstring: None,
                imports: Vec::new(),
            },
            rag_context: RagContext {
                context_before: None,
                context_after: None,
            },
        };

        let item2 = item1.clone();

        let hash1 = semantic_hash_item(&item1).unwrap();
        let hash2 = semantic_hash_item(&item2).unwrap();

        assert_eq!(hash1, hash2, "Identical items should have identical hashes");
    }

    #[test]
    fn test_hamming_distance() {
        let hash1 = "blake3:0000000000000000000000000000000000000000000000000000000000000000";
        let hash2 = "blake3:0000000000000000000000000000000000000000000000000000000000000001";

        let distance = hash_hamming_distance(hash1, hash2).unwrap();
        assert_eq!(
            distance, 1,
            "Single bit difference should give distance of 1"
        );
    }
}
