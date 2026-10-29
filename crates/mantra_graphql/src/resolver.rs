use crate::types::GraphQLValue;
use std::collections::HashMap;
use std::sync::Arc;

/// Execution context passed to resolver functions.
#[derive(Clone)]
pub struct ResolveContext<'a> {
    pub parent: Option<&'a GraphQLValue>,
    pub args: HashMap<String, GraphQLValue>,
    pub field_name: String,
    pub user_context: Arc<HashMap<String, String>>,
}

impl<'a> ResolveContext<'a> {
    pub fn get_arg(&self, name: &str) -> Option<&GraphQLValue> {
        self.args.get(name)
    }

    pub fn get_arg_str(&self, name: &str) -> Option<&str> {
        match self.args.get(name)? {
            GraphQLValue::String(s) => Some(s.as_str()),
            GraphQLValue::Enum(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn get_arg_int(&self, name: &str) -> Option<i64> {
        match self.args.get(name)? {
            GraphQLValue::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn get_arg_bool(&self, name: &str) -> Option<bool> {
        match self.args.get(name)? {
            GraphQLValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_context_value(&self, key: &str) -> Option<&str> {
        self.user_context.get(key).map(|s| s.as_str())
    }
}

pub type ResolverFn = Arc<dyn Fn(&ResolveContext) -> Result<GraphQLValue, String> + Send + Sync>;

#[derive(Default, Clone)]
pub struct ResolverRegistry {
    resolvers: HashMap<(String, String), ResolverFn>,
}

impl ResolverRegistry {
    pub fn new() -> Self {
        ResolverRegistry {
            resolvers: HashMap::new(),
        }
    }

    pub fn register<F>(&mut self, type_name: &str, field_name: &str, f: F)
    where
        F: Fn(&ResolveContext) -> Result<GraphQLValue, String> + Send + Sync + 'static,
    {
        self.resolvers.insert((type_name.to_string(), field_name.to_string()), Arc::new(f));
    }

    pub fn get(&self, type_name: &str, field_name: &str) -> Option<&ResolverFn> {
        self.resolvers.get(&(type_name.to_string(), field_name.to_string()))
    }
}
