use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the type of code item extracted
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemKind {
    Function,
    Method,
    Impl,
    Trait,
    Module,
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemKind::Function => write!(f, "function"),
            ItemKind::Method => write!(f, "method"),
            ItemKind::Impl => write!(f, "impl"),
            ItemKind::Trait => write!(f, "trait"),
            ItemKind::Module => write!(f, "module"),
        }
    }
}

/// Contains project context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub repo_url: String,
    pub commit_hash: String,
    pub file_path: String,
}

/// Contains metadata about the extracted item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMeta {
    pub kind: ItemKind,
    pub name: String,
    pub fully_qualified_name: String,
    pub start_line: u32,
    pub end_line: u32,
}

/// Contains the normalized content and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub signature: String,
    pub body_normalized: String,
    pub semantic_hash: String,
    pub docstring: Option<String>,
    pub imports: Vec<String>,
}

/// Contains contextual information for RAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagContext {
    pub context_before: Option<String>,
    pub context_after: Option<String>,
}

/// Represents a complete extracted code item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedItem {
    pub project_context: ProjectContext,
    pub item_meta: ItemMeta,
    pub content: Content,
    pub rag_context: RagContext,
}

/// Error types for the library
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Syn parsing error: {0}")]
    SynParse(#[from] syn::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Hashing error: {0}")]
    Hash(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Strip prefix error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),
}

/// Result type for core operations
pub type CoreResult<T> = Result<T, CoreError>;

/// Configuration for extraction process
#[derive(Debug, Clone)]
pub struct ExtractConfig {
    pub keep_docs: bool,
    pub include_context: bool,
    pub context_lines: usize,
}

impl Default for ExtractConfig {
    fn default() -> Self {
        Self {
            keep_docs: false,
            include_context: false,
            context_lines: 50,
        }
    }
}
