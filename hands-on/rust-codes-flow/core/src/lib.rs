//! RustCodeFlow Core Library
//!
//! Core library for parsing, normalizing, and extracting Rust code items
//! into structured datasets for AI training.

pub mod extractor;
pub mod hashing;
pub mod models;
pub mod normalizer;

pub use models::{
    Content, CoreError, CoreResult, ExtractConfig, ExtractedItem, ItemKind, ItemMeta,
    ProjectContext, RagContext,
};

pub use extractor::extract_items_from_file;
pub use hashing::{HashStats, hash_and_update_items};
pub use normalizer::normalize_items;
