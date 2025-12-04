use crate::models::*;
use quote::ToTokens;
use std::collections::HashMap;
use std::path::Path;
use syn::{self, visit_mut::VisitMut};

/// Visitor that extracts Rust code items from AST
pub struct ExtractVisitor<'ast> {
    pub items: Vec<ExtractedItem>,
    current_file: Option<String>,
    repo_url: Option<String>,
    commit_hash: Option<String>,
    current_module: Vec<String>,
    line_mapping: HashMap<usize, usize>, // byte pos -> line number
    source_lines: Vec<String>,
    config: &'ast ExtractConfig,
}

impl<'ast> ExtractVisitor<'ast> {
    pub fn new(config: &'ast ExtractConfig) -> Self {
        Self {
            items: Vec::new(),
            current_file: None,
            repo_url: None,
            commit_hash: None,
            current_module: Vec::new(),
            line_mapping: HashMap::new(),
            source_lines: Vec::new(),
            config,
        }
    }

    pub fn set_source(&mut self, source: &str) {
        self.line_mapping.clear();
        self.source_lines.clear();

        // Build line mapping
        let mut byte_pos = 0;
        let lines: Vec<&str> = source.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            self.line_mapping.insert(byte_pos, line_num + 1);
            self.source_lines.push(line.to_string());
            byte_pos += line.len() + 1; // +1 for newline
        }
    }

    pub fn set_file_info(&mut self, file_path: String, repo_url: String, commit_hash: String) {
        self.current_file = Some(file_path);
        self.repo_url = Some(repo_url);
        self.commit_hash = Some(commit_hash);
    }

    fn extract_context(&self, start_line: usize, end_line: usize) -> RagContext {
        if !self.config.include_context {
            return RagContext {
                context_before: None,
                context_after: None,
            };
        }

        let before_start = start_line.saturating_sub(self.config.context_lines);
        let after_end = end_line + self.config.context_lines;

        let context_before = if before_start < start_line {
            let lines: Vec<String> = self
                .source_lines
                .get(before_start.saturating_sub(1)..start_line.saturating_sub(1))
                .unwrap_or(&[])
                .to_vec();
            if lines.is_empty() {
                None
            } else {
                Some(lines.join("\n"))
            }
        } else {
            None
        };

        let context_after = if after_end > end_line {
            let max_len = self.source_lines.len();
            // simple bounds check to prevent panic
            let start = end_line.min(max_len);
            let end = after_end.min(max_len);

            if start < end {
                let lines: Vec<String> = self.source_lines[start..end].to_vec();
                Some(lines.join("\n"))
            } else {
                None
            }
        } else {
            None
        };

        RagContext {
            context_before,
            context_after,
        }
    }
}

impl<'ast> VisitMut for ExtractVisitor<'ast> {
    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        let start_line = 1; // Placeholder: Real implementation requires span-to-line mapping
        let end_line = 1;

        let item_meta = ItemMeta {
            kind: ItemKind::Function,
            name: item.sig.ident.to_string(),
            fully_qualified_name: format!("{}::{}", self.current_module.join("::"), item.sig.ident),
            start_line: start_line as u32,
            end_line: end_line as u32,
        };

        let content = Content {
            signature: item.sig.clone().into_token_stream().to_string(),
            body_normalized: item.clone().into_token_stream().to_string(),
            semantic_hash: String::new(),
            docstring: None,
            imports: Vec::new(),
        };

        let rag_context = self.extract_context(start_line, end_line);

        let project_context = ProjectContext {
            repo_url: self.repo_url.clone().unwrap_or_default(),
            commit_hash: self.commit_hash.clone().unwrap_or_default(),
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        syn::visit_mut::visit_item_fn_mut(self, item);
    }

    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        let start_line = 1;
        let end_line = 1;

        let item_meta = ItemMeta {
            kind: ItemKind::Impl,
            name: "impl".to_string(),
            fully_qualified_name: format!("{}::impl", self.current_module.join("::")),
            start_line: start_line as u32,
            end_line: end_line as u32,
        };

        let content = Content {
            signature: item.clone().into_token_stream().to_string(),
            body_normalized: item.clone().into_token_stream().to_string(),
            semantic_hash: String::new(),
            docstring: None,
            imports: Vec::new(),
        };

        let rag_context = self.extract_context(start_line, end_line);

        let project_context = ProjectContext {
            repo_url: self.repo_url.clone().unwrap_or_default(),
            commit_hash: self.commit_hash.clone().unwrap_or_default(),
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        syn::visit_mut::visit_item_impl_mut(self, item);
    }

    fn visit_item_trait_mut(&mut self, item: &mut syn::ItemTrait) {
        let start_line = 1;
        let end_line = 1;

        let item_meta = ItemMeta {
            kind: ItemKind::Trait,
            name: item.ident.to_string(),
            fully_qualified_name: format!("{}::{}", self.current_module.join("::"), item.ident),
            start_line: start_line as u32,
            end_line: end_line as u32,
        };

        let content = Content {
            signature: item.clone().into_token_stream().to_string(),
            body_normalized: item.clone().into_token_stream().to_string(),
            semantic_hash: String::new(),
            docstring: None,
            imports: Vec::new(),
        };

        let rag_context = self.extract_context(start_line, end_line);

        let project_context = ProjectContext {
            repo_url: self.repo_url.clone().unwrap_or_default(),
            commit_hash: self.commit_hash.clone().unwrap_or_default(),
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        syn::visit_mut::visit_item_trait_mut(self, item);
    }

    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        let start_line = 1;
        let end_line = 1;

        self.current_module.push(item.ident.to_string());

        let item_meta = ItemMeta {
            kind: ItemKind::Module,
            name: item.ident.to_string(),
            fully_qualified_name: self.current_module.join("::"),
            start_line: start_line as u32,
            end_line: end_line as u32,
        };

        let content = Content {
            signature: item.clone().into_token_stream().to_string(),
            body_normalized: item.clone().into_token_stream().to_string(),
            semantic_hash: String::new(),
            docstring: None,
            imports: Vec::new(),
        };

        let rag_context = self.extract_context(start_line, end_line);

        let project_context = ProjectContext {
            repo_url: self.repo_url.clone().unwrap_or_default(),
            commit_hash: self.commit_hash.clone().unwrap_or_default(),
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        syn::visit_mut::visit_item_mod_mut(self, item);
        self.current_module.pop();
    }
}

/// Main extraction function taking a raw string source
pub fn extract_items_from_source(
    source: &str,
    file_path: String,
    repo_url: String,
    commit_hash: String,
    config: &ExtractConfig,
) -> CoreResult<Vec<ExtractedItem>> {
    let parsed_file = syn::parse_file(source)?;

    let mut visitor = ExtractVisitor::new(config);
    visitor.set_source(source);
    visitor.set_file_info(file_path, repo_url, commit_hash);

    let mut parsed_file_mut = parsed_file;
    visitor.visit_file_mut(&mut parsed_file_mut);

    Ok(visitor.items)
}

/// Extract items from a file path using known "rustcodeflow_" pattern to normalize paths
pub fn extract_items_from_file(
    file_path: &Path,
    repo_url: String,
    commit_hash: String,
    config: &ExtractConfig,
) -> CoreResult<Vec<ExtractedItem>> {
    let source = std::fs::read_to_string(file_path)?;

    // Heuristic:
    // Split path into components. Find the component starting with "rustcodeflow_".
    // Truncate everything before it.
    // Strip "rustcodeflow_" prefix from that specific component to get clean repo name.

    let components: Vec<_> = file_path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect();

    let start_index = components
        .iter()
        .position(|c| c.starts_with("rustcodeflow_"));

    let relative_path_str = if let Some(idx) = start_index {
        // e.g. ["tmp", "x", "rustcodeflow_rust", "src", "lib.rs"] -> idx=2
        // repo_dir = "rustcodeflow_rust" -> "rust"
        let repo_dir = &components[idx];
        let clean_repo_name = repo_dir.strip_prefix("rustcodeflow_").unwrap_or(repo_dir);

        let mut parts = vec![clean_repo_name];
        // Append all subsequent components
        parts.extend(components.iter().skip(idx + 1).map(|s| s.as_ref()));
        parts.join("/")
    } else {
        // Fallback: If pattern not found (e.g. local run), just normalize slashes
        file_path.to_string_lossy().replace('\\', "/")
    };

    extract_items_from_source(&source, relative_path_str, repo_url, commit_hash, config)
}
