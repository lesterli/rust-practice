# RustCodesFlow: Rust Codes Dataset Generator

RustCodesFlow is a fast CLI tool that converts any public Rust GitHub repository into a clean JSONL dataset optimized for training AI coding agents.

RustCodesFlow transforms Rust source code into normalized datasets that teach AI the logic and structure of code rather than arbitrary formatting differences. By anonymizing identifiers and normalizing formatting, it maximizes learning efficiency while preserving semantic meaning.

## Quick Start

```bash
# Install
git clone https://github.com/lesterli/rust-practice.git
cd rust-practice/hands-on/rust-codes-flow
cargo install --path cli

# Convert a repository to JSONL dataset
rustcodesflow https://github.com/rust-lang/rust --output rust_compiler_dataset.jsonl
```

## Key Features

- ⚡ **Lightning Fast**: Parallel processing with rayon for optimal performance
- 🎯 **AI-Optimized**: Identifier anonymization (var_0, var_1, T0, T1) for better learning
- 🏗️ **Full AST Parsing**: Complete extraction using syn with full fidelity
- 📊 **Rich Metadata**: Semantic hashes, context, imports, and signatures
- 📝 **Documentation Options**: Optional docstring preservation

## Architecture

### Structure

```
rust-codes-flow/              # Workspace root
├── Cargo.toml               # Workspace definition
├── core/                    # Core logic library
│   ├── src/
│   │   ├── lib.rs           # Library entry point
│   │   ├── extractor.rs     # Syn parsing & AST traversal
│   │   ├── normalizer.rs    # AST manipulation & identifier anonymization
│   │   ├── hashing.rs       # BLAKE3 semantic hashing
│   │   └── models.rs        # Data structures & JSON schemas
│   └── Cargo.toml
├── cli/                     # Binary entry point
│   ├── src/
│   │   └── main.rs          # CLI logic, git integration, parallel processing
│   └── Cargo.toml
└── README.md
```

### Data Pipeline

1. **Input**: GitHub URL or local path
2. **Discovery**: Find all `.rs` files using walkdir
3. **Parse**: Full AST parsing with syn
4. **Extract**: Functions, methods, traits, impl blocks, modules
5. **Normalize**: Anonymize identifiers, remove comments, standardize formatting
6. **Hash**: BLAKE3 semantic hashing for deduplication
7. **Enrich**: Extract context and imports
8. **Output**: Stream to JSONL with progress tracking

## Usage Examples

### Basic Conversion

```bash
# Process a GitHub repository
rustcodesflow https://github.com/serde-rs/serde --output serde_dataset.jsonl

# Process local repository
rustcodesflow ./my-rust-project --output local_dataset.jsonl
```

### Command Line Options

- `--output, -o`: Output JSONL file path (required)
- `--keep-docs`: Preserve docstring comments in output
- `--full-context`: Include 50 lines of context before/after each item
- `--threads`: Number of parallel threads (default: logical CPUs)

## JSONL Output Schema

Each line in the output file represents one extracted code item:

```json
{
  "project_context": {
    "repo_url": "https://github.com/rust-lang/rust",
    "commit_hash": "a1b2c3d4e5f6789...",
    "file_path": "src/libstd/io/mod.rs"
  },
  "item_meta": {
    "kind": "function",
    "name": "read_to_end",
    "fully_qualified_name": "std::io::Read::read_to_end",
    "start_line": 124,
    "end_line": 156
  },
  "content": {
    "signature": "fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>",
    "body_normalized": "fn read_to_end(var0: &mut self, var1: &mut Vec<u8>) -> Result<usize> { ... }",
    "semantic_hash": "8f4343460d268d83a3c8f3c9e4d5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2",
    "docstring": "Reads entire input into this buffer.", 
    "imports": [
      "use std::io::{self, BufRead, Result}",
      "use crate::io::BufReader"
    ]
  },
  "rag_context": {
    "context_before": "impl<R: Read> BufReader<R> {",
    "context_after": "    Ok(buf.len())"
  }
}
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `kind` | string | Item type: `function`, `method`, `impl`, `trait`, `module` |
| `fully_qualified_name` | string | Best-effort qualified name |
| `body_normalized` | string | Source code with normalized identifiers |
| `semantic_hash` | string | BLAKE3 hash of normalized content for deduplication |
| `context_before/after` | string | Surrounding source lines (optional) |
| `imports` | array | Direct imports used by this item |

## Use Cases

- **LLM Training**: Train models on normalized Rust code patterns
- **RAG Systems**: Build retrieval-augmented generation for code intelligence
- **Code Analysis**: Analyze architectural patterns across repositories
- **Autocomplete**: Generate training data for AI-powered code completion

## License

MIT License - see LICENSE file for details.

---

Built with ❤️ for the Rust and AI communities.
