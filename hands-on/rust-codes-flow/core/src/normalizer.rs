use crate::models::*;
use std::collections::HashMap;
use syn::{self, visit_mut::VisitMut};

/// Scope-aware identifier normalizer
/// Maps identifiers to neutral names like var_0, var_1, T0, T1, etc.
pub struct Normalizer<'ast> {
    // Stack of scope mappings: original_name -> normalized_name
    scope_stack: Vec<HashMap<String, String>>,
    // Counter for generating normalized names
    var_counter: usize,
    type_counter: usize,
    // Configuration
    config: &'ast ExtractConfig,
}

impl<'ast> Normalizer<'ast> {
    pub fn new(config: &'ast ExtractConfig) -> Self {
        Self {
            scope_stack: Vec::new(),
            var_counter: 0,
            type_counter: 0,
            config,
        }
    }

    pub fn normalize_file(&mut self, _file: &mut syn::File) -> String {
        // Reset counters for each file
        self.var_counter = 0;
        self.type_counter = 0;
        self.scope_stack.clear();

        // For now, return a simplified implementation that doesn't require AST modification
        // In a full implementation, we would visit and modify the AST
        "".to_string()
    }

    pub fn extract_docstring(&self, _item: &syn::Item) -> Option<String> {
        if !self.config.keep_docs {
            return None;
        }

        // Simple docstring extraction - in a real implementation you'd parse attributes
        None
    }

    pub fn extract_imports(&self, _file: &syn::File) -> Vec<String> {
        // Simple import extraction - in a real implementation you'd traverse use statements
        Vec::new()
    }

    fn enter_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn normalize_identifier(&mut self, ident: &syn::Ident) -> syn::Ident {
        let original = ident.to_string();

        // Check if this identifier is already in any scope
        for scope in self.scope_stack.iter().rev() {
            if let Some(normalized) = scope.get(&original) {
                return syn::Ident::new(normalized, ident.span());
            }
        }

        // Generate new normalized name
        let normalized = if original.chars().next().map_or(false, |c| c.is_uppercase()) {
            // It's a type name
            let normalized = format!("T{}", self.type_counter);
            self.type_counter += 1;
            normalized
        } else {
            // It's a variable/function name
            let normalized = format!("var_{}", self.var_counter);
            self.var_counter += 1;
            normalized
        };

        // Store in current scope
        if let Some(current_scope) = self.scope_stack.last_mut() {
            current_scope.insert(original, normalized.clone());
        }

        syn::Ident::new(&normalized, ident.span())
    }

    fn is_local_variable(&self, ident: &syn::Ident) -> bool {
        let name = ident.to_string();

        // Skip standard library and known patterns
        if name.starts_with("std::") || name.starts_with("core::") || name.starts_with("crate::") {
            return false;
        }

        // Check if this identifier is declared in current or parent scopes
        for scope in self.scope_stack.iter().rev() {
            if scope.contains_key(&name) {
                return true;
            }
        }

        false
    }
}

impl<'ast> VisitMut for Normalizer<'ast> {
    fn visit_generics_mut(&mut self, generics: &mut syn::Generics) {
        // Normalize generic parameters
        for param in &mut generics.params {
            match param {
                syn::GenericParam::Type(type_param) => {
                    // Normalize the type parameter name
                    let original = type_param.ident.to_string();
                    let normalized = format!("T{}", self.type_counter);
                    self.type_counter += 1;

                    if let Some(current_scope) = self.scope_stack.last_mut() {
                        current_scope.insert(original, normalized.clone());
                    }

                    type_param.ident = syn::Ident::new(&normalized, type_param.ident.span());
                }
                syn::GenericParam::Lifetime(_) => {}
                syn::GenericParam::Const(_) => {}
            }
        }
    }

    fn visit_pat_ident_mut(&mut self, pat: &mut syn::PatIdent) {
        // This is a variable declaration
        let normalized = self.normalize_identifier(&pat.ident);
        pat.ident = normalized;
    }

    fn visit_expr_path_mut(&mut self, expr: &mut syn::ExprPath) {
        // Check if this is a local variable usage
        if let Some(path_seg) = expr.path.segments.first() {
            if self.is_local_variable(&path_seg.ident) {
                let normalized = self.normalize_identifier(&path_seg.ident);
                expr.path.segments[0].ident = normalized;
            }
        }

        syn::visit_mut::visit_expr_path_mut(self, expr);
    }

    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        self.enter_scope();

        // Normalize function name
        let normalized_name = self.normalize_identifier(&item.sig.ident);
        item.sig.ident = normalized_name;

        // Normalize function parameters
        for input in &mut item.sig.inputs {
            match input {
                syn::FnArg::Receiver(_) => {}
                syn::FnArg::Typed(pat_type) => {
                    self.visit_pat_mut(&mut pat_type.pat);
                }
            }
        }

        // Visit function body
        self.visit_block_mut(&mut item.block);

        self.exit_scope();
    }

    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        self.enter_scope();

        for stmt in &mut block.stmts {
            self.visit_stmt_mut(stmt);
        }

        self.exit_scope();
    }

    fn visit_item_mod_mut(&mut self, item: &mut syn::ItemMod) {
        // Normalize module name
        let normalized = self.normalize_identifier(&item.ident);
        item.ident = normalized;

        if let Some((_, items)) = &mut item.content {
            self.enter_scope();
            for item in items {
                self.visit_item_mut(item);
            }
            self.exit_scope();
        }
    }

    fn visit_item_trait_mut(&mut self, item: &mut syn::ItemTrait) {
        // Normalize trait name
        let normalized = self.normalize_identifier(&item.ident);
        item.ident = normalized;

        self.enter_scope();

        // Normalize generics
        self.visit_generics_mut(&mut item.generics);

        // Visit trait items
        for item in &mut item.items {
            self.visit_trait_item_mut(item);
        }

        self.exit_scope();
    }

    fn visit_item_impl_mut(&mut self, item: &mut syn::ItemImpl) {
        self.enter_scope();

        // Visit impl block
        self.visit_generics_mut(&mut item.generics);

        // Handle different impl types - simplified approach
        // Just visit the self type regardless of trait impl or not
        self.visit_type_mut(&mut item.self_ty);

        for item in &mut item.items {
            self.visit_impl_item_mut(item);
        }

        self.exit_scope();
    }
}

/// Normalize a complete ExtractedItem
pub fn normalize_item(item: &mut ExtractedItem, config: &ExtractConfig) -> CoreResult<()> {
    let _normalizer = Normalizer::new(config);

    // For now, just keep the original content
    // In a full implementation, we would:
    // 1. Parse the body back into AST
    // 2. Normalize the AST
    // 3. Convert back to string

    // Update the item with basic info
    item.content.docstring = None;
    item.content.imports = Vec::new();

    Ok(())
}

/// Batch normalize multiple items
pub fn normalize_items(items: &mut [ExtractedItem], config: &ExtractConfig) -> CoreResult<()> {
    for item in items {
        normalize_item(item, config)?;
    }
    Ok(())
}
