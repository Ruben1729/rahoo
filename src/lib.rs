extern crate syn;

use syn::{visit::Visit, ItemFn};
use std::fs;

struct FunctionVisitor {
    matches: Vec<String>
}

impl <'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        let function_signature = format!("{}", quote::quote! { #node.sig } );
        if function_signature.contains("usize") && function_signature.contains("->") {
            self.matches.push(function_signature);
        }
    }
}

fn search_in_file(file: &str) -> Vec<String> {
    let content = fs::read_to_string(file).expect("Unable to read file when searching.");
    let syntax_tree: syn::File = syn::parse_str(&content).expect("Unable to parse file");
    let mut visitor = FunctionVisitor { matches: Vec::new() };
    visitor.visit_file(&syntax_tree);
    visitor.matches
}

fn test(testing: usize) -> usize {
    testing
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visit_file() {
        let matches = search_in_file("~/Projects/rahoo/src/lib.rs");
        for signature in matches {
            println!("{}", signature);
        }
        assert!(false);
    }
}
