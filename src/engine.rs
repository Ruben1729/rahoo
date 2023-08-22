use syn::{visit::Visit, ItemFn, FnArg, ReturnType, ItemImpl, ImplItem, Visibility};
use std::{fmt, fs};
use std::path::Path;
use crate::QueryInfo;

pub struct MatchBuilder {
    pub matches: Vec<FnInfo>,
    query: QueryInfo
}

impl MatchBuilder {
    pub fn new(query: &str) -> Self {
        MatchBuilder {
            matches: Vec::new(),
            query: QueryInfo::from(query).expect("Unable to parse query")
        }
    }
}

impl <'ast> Visit<'ast> for MatchBuilder {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        if let Some(info) = self.extract_signature(&node.sig, &node.vis) {
            self.matches.push(info);
        }
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        for item in &node.items {
            if let ImplItem::Method(method) = item {
                if let Some(info) = self.extract_signature(&method.sig, &method.vis) {
                    self.matches.push(info);
                }
            }
        }
    }
}

impl MatchBuilder {
    fn extract_signature(&self, sig: &syn::Signature, vis: &Visibility) -> Option<FnInfo> {
        let visibility = match vis {
            Visibility::Public(_) => "public",
            Visibility::Crate(_) => "crate",
            Visibility::Restricted(_) => "restricted",
            Visibility::Inherited => "inherited",
        }.to_string();

        let inputs = sig.inputs.iter().map(|arg| {
            match arg {
                FnArg::Typed(t) => format!("{}", quote::quote! { #t }),
                FnArg::Receiver(r) => format!("{}", quote::quote! { #r }),
            }
        }).collect::<Vec<String>>();

        if inputs.len() != self.query.inputs().len() {
            return None
        }

        let mut stack = self.query.inputs().clone();

        while stack.len() > 0 {
            let item = stack.remove(0);
            inputs.iter().find(|&x| x.contains(&item));
        }

        let output = match &sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, t) => Some(format!("{}", quote::quote! { #t })),
        };

        if output != *self.query.output() {
            return None
        }

        Some(FnInfo {
            name: sig.ident.to_string(),
            inputs,
            output,
            visibility,
        })
    }

    pub fn visit_node<P: AsRef<Path>>(&mut self, path: P) {
        if path.as_ref().is_file() {
            let content = fs::read_to_string(path)
                .expect("Failed to read the file");
            let syntax_tree: syn::File = syn::parse_str(&content)
                .expect("Failed to parse the content");
            self.visit_file(&syntax_tree);
        } else {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        self.visit_node(entry.path());
                    }
                }
            }
        }
    }
}

pub struct FnInfo {
    name: String,
    inputs: Vec<String>,
    output: Option<String>,
    visibility: String
}

impl FnInfo {
    pub fn name(&self) -> & String {
        &self.name
    }

    pub fn inputs(&self) -> & Vec<String> {
        &self.inputs
    }

    pub fn output(&self) -> & Option<String> {
        &self.output
    }

    pub fn visibility(&self) -> & String {
        &self.visibility
    }
}

impl fmt::Display for FnInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Constructing the function arguments from inputs
        let input_str = self.inputs.join(", ");

        // If the output type is present, format it with "->", otherwise it'll be an empty string
        let output_str = match &self.output {
            Some(out) => format!("-> {}", out),
            None => String::new(),
        };

        // Formatting the whole function signature
        write!(f, "fn {}({}) {}", self.name, input_str, output_str)
    }
}