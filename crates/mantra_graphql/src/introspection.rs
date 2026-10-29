use crate::schema::{GraphQLSchema, TypeDefinition, TypeKind};
use crate::types::{FieldDefinition, GraphQLValue};
use std::collections::HashMap;

pub fn resolve_typename(type_name: &str) -> GraphQLValue {
    GraphQLValue::String(type_name.to_string())
}

pub fn resolve_schema(schema: &GraphQLSchema) -> GraphQLValue {
    let mut schema_map = HashMap::new();

    let mut query_type_map = HashMap::new();
    query_type_map.insert("name".to_string(), GraphQLValue::String(schema.query_type.clone()));
    schema_map.insert("queryType".to_string(), GraphQLValue::Object(query_type_map));

    if let Some(ref mut_name) = schema.mutation_type {
        let mut m_map = HashMap::new();
        m_map.insert("name".to_string(), GraphQLValue::String(mut_name.clone()));
        schema_map.insert("mutationType".to_string(), GraphQLValue::Object(m_map));
    } else {
        schema_map.insert("mutationType".to_string(), GraphQLValue::Null);
    }

    if let Some(ref sub_name) = schema.subscription_type {
        let mut s_map = HashMap::new();
        s_map.insert("name".to_string(), GraphQLValue::String(sub_name.clone()));
        schema_map.insert("subscriptionType".to_string(), GraphQLValue::Object(s_map));
    } else {
        schema_map.insert("subscriptionType".to_string(), GraphQLValue::Null);
    }

    let mut types_list = Vec::new();
    let mut sorted_keys: Vec<&String> = schema.types.keys().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        let t_def = &schema.types[key];
        types_list.push(format_type_definition(t_def));
    }

    schema_map.insert("types".to_string(), GraphQLValue::List(types_list));
    GraphQLValue::Object(schema_map)
}

pub fn resolve_type(schema: &GraphQLSchema, name: &str) -> GraphQLValue {
    match schema.get_type(name) {
        Some(t_def) => format_type_definition(t_def),
        None => GraphQLValue::Null,
    }
}

fn format_type_definition(t_def: &TypeDefinition) -> GraphQLValue {
    let mut map = HashMap::new();
    map.insert("name".to_string(), GraphQLValue::String(t_def.name().to_string()));

    let kind_str = match t_def.kind() {
        TypeKind::Scalar => "SCALAR",
        TypeKind::Object => "OBJECT",
        TypeKind::Interface => "INTERFACE",
        TypeKind::Union => "UNION",
        TypeKind::Enum => "ENUM",
        TypeKind::InputObject => "INPUT_OBJECT",
        TypeKind::List => "LIST",
        TypeKind::NonNull => "NON_NULL",
    };
    map.insert("kind".to_string(), GraphQLValue::String(kind_str.to_string()));

    if let Some(desc) = t_def.description() {
        map.insert("description".to_string(), GraphQLValue::String(desc.to_string()));
    } else {
        map.insert("description".to_string(), GraphQLValue::Null);
    }

    match t_def {
        TypeDefinition::Object(o) => {
            let mut fields = Vec::new();
            let mut f_names: Vec<&String> = o.fields.keys().collect();
            f_names.sort();
            for fname in f_names {
                let f = &o.fields[fname];
                fields.push(format_field(f));
            }
            map.insert("fields".to_string(), GraphQLValue::List(fields));
        }
        TypeDefinition::Interface(i) => {
            let mut fields = Vec::new();
            let mut f_names: Vec<&String> = i.fields.keys().collect();
            f_names.sort();
            for fname in f_names {
                let f = &i.fields[fname];
                fields.push(format_field(f));
            }
            map.insert("fields".to_string(), GraphQLValue::List(fields));
        }
        TypeDefinition::Enum(e) => {
            let mut enum_values = Vec::new();
            for v in &e.values {
                let mut ev_map = HashMap::new();
                ev_map.insert("name".to_string(), GraphQLValue::String(v.name.clone()));
                enum_values.push(GraphQLValue::Object(ev_map));
            }
            map.insert("enumValues".to_string(), GraphQLValue::List(enum_values));
        }
        _ => {
            map.insert("fields".to_string(), GraphQLValue::Null);
        }
    }

    GraphQLValue::Object(map)
}

fn format_field(f: &FieldDefinition) -> GraphQLValue {
    let mut map = HashMap::new();
    map.insert("name".to_string(), GraphQLValue::String(f.name.clone()));
    if let Some(ref d) = f.description {
        map.insert("description".to_string(), GraphQLValue::String(d.clone()));
    } else {
        map.insert("description".to_string(), GraphQLValue::Null);
    }

    let mut type_map = HashMap::new();
    type_map.insert("name".to_string(), GraphQLValue::String(f.field_type.to_string()));
    map.insert("type".to_string(), GraphQLValue::Object(type_map));

    let mut args = Vec::new();
    for a in &f.arguments {
        let mut arg_map = HashMap::new();
        arg_map.insert("name".to_string(), GraphQLValue::String(a.name.clone()));
        let mut arg_type_map = HashMap::new();
        arg_type_map.insert("name".to_string(), GraphQLValue::String(a.arg_type.to_string()));
        arg_map.insert("type".to_string(), GraphQLValue::Object(arg_type_map));
        args.push(GraphQLValue::Object(arg_map));
    }
    map.insert("args".to_string(), GraphQLValue::List(args));

    GraphQLValue::Object(map)
}
