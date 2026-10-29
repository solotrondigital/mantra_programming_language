use crate::types::GraphQLValue;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphQLError {
    pub message: String,
    pub path: Vec<String>,
}

impl GraphQLError {
    pub fn new(message: &str) -> Self {
        GraphQLError {
            message: message.to_string(),
            path: Vec::new(),
        }
    }

    pub fn with_path(message: &str, path: Vec<String>) -> Self {
        GraphQLError {
            message: message.to_string(),
            path,
        }
    }

    pub fn to_json(&self) -> String {
        let escaped = self
            .message
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n");
        if self.path.is_empty() {
            format!("{{\"message\":\"{}\"}}", escaped)
        } else {
            let path_items: Vec<String> = self.path.iter().map(|p| format!("\"{}\"", p)).collect();
            format!(
                "{{\"message\":\"{}\",\"path\":[{}]}}",
                escaped,
                path_items.join(",")
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphQLResponse {
    pub data: Option<GraphQLValue>,
    pub errors: Vec<GraphQLError>,
}

impl GraphQLResponse {
    pub fn ok(data: GraphQLValue) -> Self {
        GraphQLResponse {
            data: Some(data),
            errors: Vec::new(),
        }
    }

    pub fn error(message: &str) -> Self {
        GraphQLResponse {
            data: None,
            errors: vec![GraphQLError::new(message)],
        }
    }

    pub fn errors(errs: Vec<String>) -> Self {
        GraphQLResponse {
            data: None,
            errors: errs.into_iter().map(|e| GraphQLError::new(&e)).collect(),
        }
    }

    pub fn to_json(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref d) = self.data {
            parts.push(format!("\"data\":{}", d.to_json()));
        } else {
            parts.push("\"data\":null".to_string());
        }

        if !self.errors.is_empty() {
            let err_strs: Vec<String> = self.errors.iter().map(|e| e.to_json()).collect();
            parts.push(format!("\"errors\":[{}]", err_strs.join(",")));
        }

        format!("{{{}}}", parts.join(","))
    }
}

