use crate::models::*;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{self, visit_mut::VisitMut};

/// Visitor that extracts Rust code items from AST
pub struct ExtractVisitor<'ast> {
    pub items: Vec<ExtractedItem>,
    current_file: Option<String>,
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

    pub fn set_file_info(&mut self, file_path: String, _repo_url: String, _commit_hash: String) {
        self.current_file = Some(file_path);
        // We'll use this in item creation
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
            let lines: Vec<String> = self.source_lines
                [before_start.saturating_sub(1)..start_line.saturating_sub(1)]
                .to_vec();
            Some(lines.join("\n"))
        } else {
            None
        };

        let context_after = if after_end > end_line && after_end <= self.source_lines.len() {
            let lines: Vec<String> =
                self.source_lines[end_line..after_end.saturating_sub(1)].to_vec();
            Some(lines.join("\n"))
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
        // Use simpler approach - just use line 1 for now to get it compiling
        // In a full implementation, we'd use proper span handling
        let start_line = 1;
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
            semantic_hash: String::new(), // Will be filled by hashing module
            docstring: None,              // Will be filled by normalizer
            imports: Vec::new(),          // Will be filled by normalizer
        };

        let rag_context = self.extract_context(start_line, end_line);

        let project_context = ProjectContext {
            repo_url: String::new(),    // Will be filled by CLI
            commit_hash: String::new(), // Will be filled by CLI
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        // Continue visiting children
        syn::visit_mut::visit_item_fn_mut(self, item);
    }

    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        // Use simpler approach - just use line 1 for now to get it compiling
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
            repo_url: String::new(),
            commit_hash: String::new(),
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
        // Use simpler approach - just use line 1 for now to get it compiling
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
            repo_url: String::new(),
            commit_hash: String::new(),
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
        // Use simpler approach - just use line 1 for now to get it compiling
        let start_line = 1;
        let end_line = 1;

        // Track current module for nested items
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
            repo_url: String::new(),
            commit_hash: String::new(),
            file_path: self.current_file.clone().unwrap_or_default(),
        };

        self.items.push(ExtractedItem {
            project_context,
            item_meta,
            content,
            rag_context,
        });

        syn::visit_mut::visit_item_mod_mut(self, item);

        // Pop module when done
        self.current_module.pop();
    }
}

/// Main extraction function
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

/// Extract items from a file path
pub fn extract_items_from_file(
    file_path: &std::path::Path,
    repo_url: String,
    commit_hash: String,
    config: &ExtractConfig,
) -> CoreResult<Vec<ExtractedItem>> {
    let source = std::fs::read_to_string(file_path)?;
    let file_path_str = file_path.to_string_lossy().to_string();

    extract_items_from_source(&source, file_path_str, repo_url, commit_hash, config)
}
