use crate::resolver::{ResolveContext, ResolverRegistry};
use crate::types::{FieldDefinition, GraphQLValue, InputFieldDefinition};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValueDefinition {
    pub name: String,
    pub description: Option<String>,
    pub deprecation_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType {
    pub name: String,
    pub description: Option<String>,
    pub fields: HashMap<String, FieldDefinition>,
    pub interfaces: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType {
    pub name: String,
    pub description: Option<String>,
    pub fields: HashMap<String, FieldDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType {
    pub name: String,
    pub description: Option<String>,
    pub types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: String,
    pub description: Option<String>,
    pub values: Vec<EnumValueDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectType {
    pub name: String,
    pub description: Option<String>,
    pub fields: HashMap<String, InputFieldDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

#[derive(Debug, Clone)]
pub enum TypeDefinition {
    Scalar(ScalarType),
    Object(ObjectType),
    Interface(InterfaceType),
    Union(UnionType),
    Enum(EnumType),
    InputObject(InputObjectType),
}

impl TypeDefinition {
    pub fn name(&self) -> &str {
        match self {
            TypeDefinition::Scalar(s) => &s.name,
            TypeDefinition::Object(o) => &o.name,
            TypeDefinition::Interface(i) => &i.name,
            TypeDefinition::Union(u) => &u.name,
            TypeDefinition::Enum(e) => &e.name,
            TypeDefinition::InputObject(io) => &io.name,
        }
    }

    pub fn description(&self) -> Option<&str> {
        match self {
            TypeDefinition::Scalar(s) => s.description.as_deref(),
            TypeDefinition::Object(o) => o.description.as_deref(),
            TypeDefinition::Interface(i) => i.description.as_deref(),
            TypeDefinition::Union(u) => u.description.as_deref(),
            TypeDefinition::Enum(e) => e.description.as_deref(),
            TypeDefinition::InputObject(io) => io.description.as_deref(),
        }
    }

    pub fn kind(&self) -> TypeKind {
        match self {
            TypeDefinition::Scalar(_) => TypeKind::Scalar,
            TypeDefinition::Object(_) => TypeKind::Object,
            TypeDefinition::Interface(_) => TypeKind::Interface,
            TypeDefinition::Union(_) => TypeKind::Union,
            TypeDefinition::Enum(_) => TypeKind::Enum,
            TypeDefinition::InputObject(_) => TypeKind::InputObject,
        }
    }
}

pub struct GraphQLSchema {
    pub query_type: String,
    pub mutation_type: Option<String>,
    pub subscription_type: Option<String>,
    pub types: HashMap<String, TypeDefinition>,
    pub resolvers: ResolverRegistry,
}

impl GraphQLSchema {
    pub fn new(query_type: &str) -> Self {
        let mut schema = GraphQLSchema {
            query_type: query_type.to_string(),
            mutation_type: None,
            subscription_type: None,
            types: HashMap::new(),
            resolvers: ResolverRegistry::new(),
        };

        // Register default built-in scalars
        for name in &["Int", "Float", "String", "Boolean", "ID"] {
            schema.add_type(TypeDefinition::Scalar(ScalarType {
                name: name.to_string(),
                description: Some(format!("The {} scalar type", name)),
            }));
        }

        schema
    }

    pub fn add_type(&mut self, type_def: TypeDefinition) {
        self.types.insert(type_def.name().to_string(), type_def);
    }

    pub fn get_type(&self, name: &str) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    pub fn get_object(&self, name: &str) -> Option<&ObjectType> {
        match self.types.get(name) {
            Some(TypeDefinition::Object(o)) => Some(o),
            _ => None,
        }
    }

    pub fn to_sdl(&self) -> String {
        let mut out = String::new();

        // Schema block
        out.push_str("schema {\n");
        out.push_str(&format!("  query: {}\n", self.query_type));
        if let Some(ref m) = self.mutation_type {
            out.push_str(&format!("  mutation: {}\n", m));
        }
        if let Some(ref s) = self.subscription_type {
            out.push_str(&format!("  subscription: {}\n", s));
        }
        out.push_str("}\n\n");

        let mut sorted_keys: Vec<&String> = self.types.keys().collect();
        sorted_keys.sort();

        for key in sorted_keys {
            if matches!(key.as_str(), "Int" | "Float" | "String" | "Boolean" | "ID") {
                continue;
            }
            let type_def = &self.types[key];
            match type_def {
                TypeDefinition::Scalar(s) => {
                    out.push_str(&format!("scalar {}\n\n", s.name));
                }
                TypeDefinition::Object(o) => {
                    let implements = if o.interfaces.is_empty() {
                        String::new()
                    } else {
                        format!(" implements {}", o.interfaces.join(" & "))
                    };
                    out.push_str(&format!("type {}{} {{\n", o.name, implements));
                    let mut field_names: Vec<&String> = o.fields.keys().collect();
                    field_names.sort();
                    for fname in field_names {
                        let f = &o.fields[fname];
                        let args = if f.arguments.is_empty() {
                            String::new()
                        } else {
                            let arg_strs: Vec<String> = f
                                .arguments
                                .iter()
                                .map(|a| format!("{}: {}", a.name, a.arg_type))
                                .collect();
                            format!("({})", arg_strs.join(", "))
                        };
                        out.push_str(&format!("  {}{}: {}\n", f.name, args, f.field_type));
                    }
                    out.push_str("}\n\n");
                }
                TypeDefinition::Interface(i) => {
                    out.push_str(&format!("interface {} {{\n", i.name));
                    let mut field_names: Vec<&String> = i.fields.keys().collect();
                    field_names.sort();
                    for fname in field_names {
                        let f = &i.fields[fname];
                        out.push_str(&format!("  {}: {}\n", f.name, f.field_type));
                    }
                    out.push_str("}\n\n");
                }
                TypeDefinition::Union(u) => {
                    out.push_str(&format!("union {} = {}\n\n", u.name, u.types.join(" | ")));
                }
                TypeDefinition::Enum(e) => {
                    out.push_str(&format!("enum {} {{\n", e.name));
                    for v in &e.values {
                        out.push_str(&format!("  {}\n", v.name));
                    }
                    out.push_str("}\n\n");
                }
                TypeDefinition::InputObject(io) => {
                    out.push_str(&format!("input {} {{\n", io.name));
                    let mut field_names: Vec<&String> = io.fields.keys().collect();
                    field_names.sort();
                    for fname in field_names {
                        let f = &io.fields[fname];
                        out.push_str(&format!("  {}: {}\n", f.name, f.field_type));
                    }
                    out.push_str("}\n\n");
                }
            }
        }

        out.trim().to_string()
    }
}

pub struct SchemaBuilder {
    query_type: String,
    mutation_type: Option<String>,
    subscription_type: Option<String>,
    types: HashMap<String, TypeDefinition>,
    resolvers: ResolverRegistry,
}

impl SchemaBuilder {
    pub fn new(query_type: &str) -> Self {
        let mut builder = SchemaBuilder {
            query_type: query_type.to_string(),
            mutation_type: None,
            subscription_type: None,
            types: HashMap::new(),
            resolvers: ResolverRegistry::new(),
        };

        for name in &["Int", "Float", "String", "Boolean", "ID"] {
            builder.types.insert(
                name.to_string(),
                TypeDefinition::Scalar(ScalarType {
                    name: name.to_string(),
                    description: Some(format!("The {} scalar type", name)),
                }),
            );
        }

        builder
    }

    pub fn mutation(mut self, mutation_type: &str) -> Self {
        self.mutation_type = Some(mutation_type.to_string());
        self
    }

    pub fn subscription(mut self, subscription_type: &str) -> Self {
        self.subscription_type = Some(subscription_type.to_string());
        self
    }

    pub fn add_object(mut self, obj: ObjectType) -> Self {
        self.types.insert(obj.name.clone(), TypeDefinition::Object(obj));
        self
    }

    pub fn add_scalar(mut self, name: &str, description: Option<&str>) -> Self {
        self.types.insert(
            name.to_string(),
            TypeDefinition::Scalar(ScalarType {
                name: name.to_string(),
                description: description.map(|s| s.to_string()),
            }),
        );
        self
    }

    pub fn add_enum(mut self, name: &str, values: Vec<&str>) -> Self {
        let val_defs = values
            .into_iter()
            .map(|v| EnumValueDefinition {
                name: v.to_string(),
                description: None,
                deprecation_reason: None,
            })
            .collect();
        self.types.insert(
            name.to_string(),
            TypeDefinition::Enum(EnumType {
                name: name.to_string(),
                description: None,
                values: val_defs,
            }),
        );
        self
    }

    pub fn add_input_object(mut self, input: InputObjectType) -> Self {
        self.types.insert(input.name.clone(), TypeDefinition::InputObject(input));
        self
    }

    pub fn resolver<F>(mut self, type_name: &str, field_name: &str, f: F) -> Self
    where
        F: Fn(&ResolveContext) -> Result<GraphQLValue, String> + Send + Sync + 'static,
    {
        self.resolvers.register(type_name, field_name, f);
        self
    }

    pub fn build(self) -> GraphQLSchema {
        GraphQLSchema {
            query_type: self.query_type,
            mutation_type: self.mutation_type,
            subscription_type: self.subscription_type,
            types: self.types,
            resolvers: self.resolvers,
        }
    }
}
