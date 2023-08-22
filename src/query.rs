use regex::Regex;

pub struct QueryInfo {
    name: String,
    inputs: Vec<String>,
    output: Option<String>,
    visibility: Option<String>
}

impl QueryInfo {
    pub fn from(query: &str) -> Option<QueryInfo> {
        let query_pattern =
            r"(?P<visibility>pub)?\s*fn\s*\((?P<inputs>[^)]*)\)(?:\s*->\s*(?P<output>.*))?";
        let re = Regex::new(query_pattern).unwrap();

        if let Some(captures) = re.captures(query) {
            let inputs_str = captures.name("inputs").map_or("", |m| m.as_str());
            let inputs = inputs_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>();

            let output = captures.name("output").map(|m| m.as_str().trim().to_string());

            let visibility = captures
                                    .name("visibility")
                                    .map(|m| m.as_str().trim().to_string());

            Some(QueryInfo { name: "".to_string(), inputs, output, visibility })
        } else {
            None
        }
    }

    pub fn name(&self) -> & String {
        &self.name
    }

    pub fn inputs(&self) -> & Vec<String> {
        &self.inputs
    }

    pub fn output(&self) -> & Option<String> {
        &self.output
    }

    pub fn visibility(&self) -> & Option<String> {
        &self.visibility
    }
}
