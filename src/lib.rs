extern crate syn;
mod engine;
mod query;

pub use self::engine::*;
pub use self::query::*;

pub fn search(file_path: &str, query: &str) -> Vec<FnInfo> {
    let mut visitor = MatchBuilder::new(query);
    visitor.visit_node(file_path.to_string());
    visitor.matches
}

#[cfg(test)]
mod tests {

    #[test]
    fn testing_current_file() {

    }
}
