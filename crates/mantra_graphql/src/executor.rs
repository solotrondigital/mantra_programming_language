use crate::ast::*;
use crate::introspection;
use crate::parser::Parser;
use crate::resolver::ResolveContext;
use crate::response::{GraphQLError, GraphQLResponse};
use crate::schema::{GraphQLSchema, TypeDefinition};
use crate::types::GraphQLValue;
use crate::validation::Validator;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GraphQLEngine {
    pub schema: GraphQLSchema,
}

impl GraphQLEngine {
    pub fn new(schema: GraphQLSchema) -> Self {
        GraphQLEngine { schema }
    }

    pub fn execute(
        &self,
        query: &str,
        variables: Option<HashMap<String, GraphQLValue>>,
        operation_name: Option<&str>,
        context: Option<HashMap<String, String>>,
    ) -> GraphQLResponse {
        // 1. Parse document
        let doc = match Parser::from_source(query) {
            Ok(d) => d,
            Err(e) => return GraphQLResponse::error(&e),
        };

        // 2. Validate document
        let mut validator = Validator::new(&self.schema);
        if let Err(errs) = validator.validate(&doc) {
            return GraphQLResponse::errors(errs);
        }

        // 3. Find fragments and operations
        let mut fragments = HashMap::new();
        let mut operations = Vec::new();

        for def in &doc.definitions {
            match def {
                Definition::Fragment(f) => {
                    fragments.insert(f.name.clone(), f.clone());
                }
                Definition::Operation(op) => {
                    operations.push(op);
                }
            }
        }

        let target_op = if let Some(op_name) = operation_name {
            match operations.iter().find(|op| op.name.as_deref() == Some(op_name)) {
                Some(op) => *op,
                None => return GraphQLResponse::error(&format!("Operation '{}' not found", op_name)),
            }
        } else if operations.len() == 1 {
            operations[0]
        } else {
            return GraphQLResponse::error(
                "Must provide operation name if query contains multiple operations",
            );
        };

        // 4. Resolve variables
        let mut resolved_vars = HashMap::new();
        let user_vars = variables.unwrap_or_default();

        for vdef in &target_op.variable_definitions {
            if let Some(val) = user_vars.get(&vdef.name) {
                resolved_vars.insert(vdef.name.clone(), val.clone());
            } else if let Some(ref default) = vdef.default_value {
                resolved_vars.insert(vdef.name.clone(), default.clone());
            }
        }
        for (k, v) in user_vars {
            resolved_vars.entry(k).or_insert(v);
        }

        let user_ctx = Arc::new(context.unwrap_or_default());

        // 5. Execute operation selection set
        let root_type_name = match target_op.operation_type {
            OperationType::Query => &self.schema.query_type,
            OperationType::Mutation => match &self.schema.mutation_type {
                Some(m) => m,
                None => return GraphQLResponse::error("Schema does not support mutations"),
            },
            OperationType::Subscription => match &self.schema.subscription_type {
                Some(s) => s,
                None => return GraphQLResponse::error("Schema does not support subscriptions"),
            },
        };

        let mut errors = Vec::new();
        let mut path = Vec::new();

        let result = self.execute_selection_set(
            root_type_name,
            None,
            &target_op.selection_set,
            &resolved_vars,
            &fragments,
            &user_ctx,
            &mut path,
            &mut errors,
        );

        if !errors.is_empty() && result == GraphQLValue::Null {
            GraphQLResponse {
                data: None,
                errors,
            }
        } else {
            GraphQLResponse {
                data: Some(result),
                errors,
            }
        }
    }

    fn execute_selection_set(
        &self,
        current_type_name: &str,
        parent: Option<&GraphQLValue>,
        selection_set: &SelectionSet,
        variables: &HashMap<String, GraphQLValue>,
        fragments: &HashMap<String, FragmentDefinition>,
        context: &Arc<HashMap<String, String>>,
        path: &mut Vec<String>,
        errors: &mut Vec<GraphQLError>,
    ) -> GraphQLValue {
        let mut response_obj = HashMap::new();

        for selection in &selection_set.selections {
            match selection {
                Selection::Field(f) => {
                    // Check directives @include and @skip
                    if !self.should_include(&f.directives, variables) {
                        continue;
                    }

                    let field_key = f.alias.as_ref().unwrap_or(&f.name).clone();
                    path.push(field_key.clone());

                    let field_value = self.resolve_field(
                        current_type_name,
                        parent,
                        f,
                        variables,
                        fragments,
                        context,
                        path,
                        errors,
                    );

                    response_obj.insert(field_key, field_value);
                    path.pop();
                }
                Selection::FragmentSpread(frag_name) => {
                    if let Some(frag) = fragments.get(frag_name) {
                        let sub_val = self.execute_selection_set(
                            &frag.type_condition,
                            parent,
                            &frag.selection_set,
                            variables,
                            fragments,
                            context,
                            path,
                            errors,
                        );
                        if let GraphQLValue::Object(map) = sub_val {
                            for (k, v) in map {
                                response_obj.insert(k, v);
                            }
                        }
                    }
                }
                Selection::InlineFragment(inline) => {
                    let type_cond = inline
                        .type_condition
                        .as_deref()
                        .unwrap_or(current_type_name);
                    let sub_val = self.execute_selection_set(
                        type_cond,
                        parent,
                        &inline.selection_set,
                        variables,
                        fragments,
                        context,
                        path,
                        errors,
                    );
                    if let GraphQLValue::Object(map) = sub_val {
                        for (k, v) in map {
                            response_obj.insert(k, v);
                        }
                    }
                }
            }
        }

        GraphQLValue::Object(response_obj)
    }

    fn resolve_field(
        &self,
        current_type_name: &str,
        parent: Option<&GraphQLValue>,
        field: &FieldSelection,
        variables: &HashMap<String, GraphQLValue>,
        fragments: &HashMap<String, FragmentDefinition>,
        context: &Arc<HashMap<String, String>>,
        path: &mut Vec<String>,
        errors: &mut Vec<GraphQLError>,
    ) -> GraphQLValue {
        // Special introspection fields
        if field.name == "__typename" {
            return introspection::resolve_typename(current_type_name);
        }
        if current_type_name == self.schema.query_type {
            if field.name == "__schema" {
                return self.resolve_subselections(
                    "__Schema",
                    &introspection::resolve_schema(&self.schema),
                    field,
                    variables,
                    fragments,
                    context,
                    path,
                    errors,
                );
            }
            if field.name == "__type" {
                let type_name_arg = field
                    .arguments
                    .iter()
                    .find(|a| a.name == "name")
                    .and_then(|a| self.evaluate_value(&a.value, variables));

                let name_str = match type_name_arg {
                    Some(GraphQLValue::String(s)) => s,
                    _ => "".to_string(),
                };

                return self.resolve_subselections(
                    "__Type",
                    &introspection::resolve_type(&self.schema, &name_str),
                    field,
                    variables,
                    fragments,
                    context,
                    path,
                    errors,
                );
            }
        }

        // Resolve arguments
        let mut resolved_args = HashMap::new();
        for arg in &field.arguments {
            if let Some(val) = self.evaluate_value(&arg.value, variables) {
                resolved_args.insert(arg.name.clone(), val);
            }
        }

        // Try registered resolver
        let raw_value = if let Some(resolver) = self.schema.resolvers.get(current_type_name, &field.name) {
            let ctx = ResolveContext {
                parent,
                args: resolved_args,
                field_name: field.name.clone(),
                user_context: Arc::clone(context),
            };
            match resolver(&ctx) {
                Ok(val) => val,
                Err(err) => {
                    errors.push(GraphQLError::with_path(&err, path.clone()));
                    GraphQLValue::Null
                }
            }
        } else if let Some(GraphQLValue::Object(map)) = parent {
            // Default property extraction from parent object
            map.get(&field.name).cloned().unwrap_or(GraphQLValue::Null)
        } else {
            GraphQLValue::Null
        };

        let target_field_type = self.get_field_type_name(current_type_name, &field.name);

        self.resolve_subselections(
            &target_field_type,
            &raw_value,
            field,
            variables,
            fragments,
            context,
            path,
            errors,
        )
    }

    fn resolve_subselections(
        &self,
        type_name: &str,
        value: &GraphQLValue,
        field: &FieldSelection,
        variables: &HashMap<String, GraphQLValue>,
        fragments: &HashMap<String, FragmentDefinition>,
        context: &Arc<HashMap<String, String>>,
        path: &mut Vec<String>,
        errors: &mut Vec<GraphQLError>,
    ) -> GraphQLValue {
        match value {
            GraphQLValue::Null => GraphQLValue::Null,
            GraphQLValue::List(list) => {
                let mut resolved_list = Vec::new();
                for (idx, item) in list.iter().enumerate() {
                    path.push(idx.to_string());
                    let resolved = self.resolve_subselections(
                        type_name,
                        item,
                        field,
                        variables,
                        fragments,
                        context,
                        path,
                        errors,
                    );
                    resolved_list.push(resolved);
                    path.pop();
                }
                GraphQLValue::List(resolved_list)
            }
            GraphQLValue::Object(_) => {
                if let Some(ref sel_set) = field.selection_set {
                    self.execute_selection_set(
                        type_name,
                        Some(value),
                        sel_set,
                        variables,
                        fragments,
                        context,
                        path,
                        errors,
                    )
                } else {
                    value.clone()
                }
            }
            scalar => scalar.clone(),
        }
    }

    fn get_field_type_name(&self, current_type_name: &str, field_name: &str) -> String {
        if let Some(TypeDefinition::Object(o)) = self.schema.types.get(current_type_name) {
            if let Some(f) = o.fields.get(field_name) {
                return f.field_type.unwrap_name().to_string();
            }
        }
        "String".to_string()
    }

    fn evaluate_value(
        &self,
        node: &ValueNode,
        variables: &HashMap<String, GraphQLValue>,
    ) -> Option<GraphQLValue> {
        match node {
            ValueNode::Literal(v) => Some(v.clone()),
            ValueNode::Variable(name) => variables.get(name).cloned(),
        }
    }

    fn should_include(&self, directives: &[Directive], variables: &HashMap<String, GraphQLValue>) -> bool {
        for dir in directives {
            if dir.name == "include" {
                if let Some(arg) = dir.arguments.iter().find(|a| a.name == "if") {
                    let val = self.evaluate_value(&arg.value, variables);
                    if let Some(GraphQLValue::Boolean(false)) = val {
                        return false;
                    }
                }
            } else if dir.name == "skip" {
                if let Some(arg) = dir.arguments.iter().find(|a| a.name == "if") {
                    let val = self.evaluate_value(&arg.value, variables);
                    if let Some(GraphQLValue::Boolean(true)) = val {
                        return false;
                    }
                }
            }
        }
        true
    }
}
