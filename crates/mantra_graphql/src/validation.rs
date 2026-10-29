use crate::ast::*;
use crate::schema::{GraphQLSchema, TypeDefinition};
use std::collections::HashMap;

pub struct Validator<'a> {
    schema: &'a GraphQLSchema,
    fragments: HashMap<String, FragmentDefinition>,
}

impl<'a> Validator<'a> {
    pub fn new(schema: &'a GraphQLSchema) -> Self {
        Validator {
            schema,
            fragments: HashMap::new(),
        }
    }

    pub fn validate(&mut self, doc: &Document) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Collect fragments
        for def in &doc.definitions {
            if let Definition::Fragment(frag) = def {
                self.fragments.insert(frag.name.clone(), frag.clone());
            }
        }

        for def in &doc.definitions {
            match def {
                Definition::Operation(op) => {
                    let root_type_name = match op.operation_type {
                        OperationType::Query => &self.schema.query_type,
                        OperationType::Mutation => {
                            match &self.schema.mutation_type {
                                Some(m) => m,
                                None => {
                                    errors.push("Schema does not support mutations".to_string());
                                    continue;
                                }
                            }
                        }
                        OperationType::Subscription => {
                            match &self.schema.subscription_type {
                                Some(s) => s,
                                None => {
                                    errors.push("Schema does not support subscriptions".to_string());
                                    continue;
                                }
                            }
                        }
                    };

                    self.validate_selection_set(root_type_name, &op.selection_set, &mut errors);
                }
                Definition::Fragment(frag) => {
                    if self.schema.get_type(&frag.type_condition).is_none() {
                        errors.push(format!(
                            "Fragment '{}' targets unknown type '{}'",
                            frag.name, frag.type_condition
                        ));
                    } else {
                        self.validate_selection_set(&frag.type_condition, &frag.selection_set, &mut errors);
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_selection_set(
        &self,
        current_type_name: &str,
        selection_set: &SelectionSet,
        errors: &mut Vec<String>,
    ) {
        let current_type = match self.schema.get_type(current_type_name) {
            Some(t) => t,
            None => {
                errors.push(format!("Cannot validate selections on unknown type '{}'", current_type_name));
                return;
            }
        };

        for sel in &selection_set.selections {
            match sel {
                Selection::Field(f) => {
                    // Introspection special fields
                    if f.name == "__typename" {
                        continue;
                    }
                    if current_type_name == self.schema.query_type && (f.name == "__schema" || f.name == "__type") {
                        continue;
                    }

                    match current_type {
                        TypeDefinition::Object(o) => {
                            if let Some(fdef) = o.fields.get(&f.name) {
                                if let Some(ref sub_sel) = f.selection_set {
                                    let field_type_name = fdef.field_type.unwrap_name();
                                    self.validate_selection_set(field_type_name, sub_sel, errors);
                                }
                            } else {
                                errors.push(format!(
                                    "Field '{}' does not exist on type '{}'",
                                    f.name, o.name
                                ));
                            }
                        }
                        TypeDefinition::Interface(i) => {
                            if let Some(fdef) = i.fields.get(&f.name) {
                                if let Some(ref sub_sel) = f.selection_set {
                                    let field_type_name = fdef.field_type.unwrap_name();
                                    self.validate_selection_set(field_type_name, sub_sel, errors);
                                }
                            } else {
                                errors.push(format!(
                                    "Field '{}' does not exist on interface '{}'",
                                    f.name, i.name
                                ));
                            }
                        }
                        _ => {
                            errors.push(format!(
                                "Cannot query field '{}' on non-object type '{}'",
                                f.name, current_type_name
                            ));
                        }
                    }
                }
                Selection::FragmentSpread(name) => {
                    if let Some(frag) = self.fragments.get(name) {
                        self.validate_selection_set(&frag.type_condition, &frag.selection_set, errors);
                    } else {
                        errors.push(format!("Unknown fragment spread '...{}'", name));
                    }
                }
                Selection::InlineFragment(inline) => {
                    let target_type = inline.type_condition.as_deref().unwrap_or(current_type_name);
                    self.validate_selection_set(target_type, &inline.selection_set, errors);
                }
            }
        }
    }
}

