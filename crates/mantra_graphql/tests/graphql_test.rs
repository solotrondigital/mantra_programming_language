use mantra_graphql::*;
use std::collections::HashMap;

#[test]
fn test_lexer_and_parser() {
    let query = r#"
        query GetUser($id: ID!, $includeEmail: Boolean = true) {
            user(id: $id) @include(if: $includeEmail) {
                id
                name
                ...UserMeta
            }
        }
        fragment UserMeta on User {
            role
        }
    "#;

    let doc = Parser::from_source(query).expect("Failed to parse document");
    assert_eq!(doc.definitions.len(), 2);
}

#[test]
fn test_schema_builder_and_sdl() {
    let mut user_fields = HashMap::new();
    user_fields.insert("id".to_string(), FieldDefinition::new("id", GraphQLType::non_null(GraphQLType::named("ID"))));
    user_fields.insert("name".to_string(), FieldDefinition::new("name", GraphQLType::named("String")));

    let mut query_fields = HashMap::new();
    query_fields.insert(
        "user".to_string(),
        FieldDefinition::new("user", GraphQLType::named("User"))
            .with_arg("id", GraphQLType::non_null(GraphQLType::named("ID"))),
    );

    let schema = SchemaBuilder::new("Query")
        .add_object(ObjectType {
            name: "User".to_string(),
            description: Some("Application user entity".to_string()),
            fields: user_fields,
            interfaces: Vec::new(),
        })
        .add_object(ObjectType {
            name: "Query".to_string(),
            description: Some("Root query".to_string()),
            fields: query_fields,
            interfaces: Vec::new(),
        })
        .build();

    let sdl = schema.to_sdl();
    assert!(sdl.contains("schema {"));
    assert!(sdl.contains("query: Query"));
    assert!(sdl.contains("type User {"));
    assert!(sdl.contains("type Query {"));
}

#[test]
fn test_engine_query_execution() {
    let mut user_fields = HashMap::new();
    user_fields.insert("id".to_string(), FieldDefinition::new("id", GraphQLType::named("ID")));
    user_fields.insert("name".to_string(), FieldDefinition::new("name", GraphQLType::named("String")));

    let mut query_fields = HashMap::new();
    query_fields.insert(
        "user".to_string(),
        FieldDefinition::new("user", GraphQLType::named("User"))
            .with_arg("id", GraphQLType::named("ID")),
    );
    query_fields.insert("version".to_string(), FieldDefinition::new("version", GraphQLType::named("String")));

    let schema = SchemaBuilder::new("Query")
        .add_object(ObjectType {
            name: "User".to_string(),
            description: None,
            fields: user_fields,
            interfaces: Vec::new(),
        })
        .add_object(ObjectType {
            name: "Query".to_string(),
            description: None,
            fields: query_fields,
            interfaces: Vec::new(),
        })
        .resolver("Query", "version", |_ctx| {
            Ok(GraphQLValue::String("1.0.0".to_string()))
        })
        .resolver("Query", "user", |ctx| {
            let id = ctx.get_arg_str("id").unwrap_or("0");
            let mut user_map = HashMap::new();
            user_map.insert("id".to_string(), GraphQLValue::String(id.to_string()));
            user_map.insert("name".to_string(), GraphQLValue::String(format!("User_{}", id)));
            Ok(GraphQLValue::Object(user_map))
        })
        .build();

    let engine = GraphQLEngine::new(schema);

    // Test query execution with arguments and aliases
    let query = r#"
        query {
            version
            u1: user(id: "42") {
                id
                name
            }
        }
    "#;

    let response = engine.execute(query, None, None, None);
    assert!(response.errors.is_empty(), "Errors: {:?}", response.errors);

    let json = response.to_json();
    assert!(json.contains("\"version\":\"1.0.0\""));
    assert!(json.contains("\"u1\":{"));
    assert!(json.contains("\"id\":\"42\""));
    assert!(json.contains("\"name\":\"User_42\""));
}

#[test]
fn test_mutation_execution() {
    let mut user_fields = HashMap::new();
    user_fields.insert("id".to_string(), FieldDefinition::new("id", GraphQLType::named("ID")));
    user_fields.insert("name".to_string(), FieldDefinition::new("name", GraphQLType::named("String")));

    let mut mutation_fields = HashMap::new();
    mutation_fields.insert(
        "createUser".to_string(),
        FieldDefinition::new("createUser", GraphQLType::named("User"))
            .with_arg("name", GraphQLType::named("String")),
    );

    let schema = SchemaBuilder::new("Query")
        .mutation("Mutation")
        .add_object(ObjectType {
            name: "User".to_string(),
            description: None,
            fields: user_fields,
            interfaces: Vec::new(),
        })
        .add_object(ObjectType {
            name: "Query".to_string(),
            description: None,
            fields: HashMap::new(),
            interfaces: Vec::new(),
        })
        .add_object(ObjectType {
            name: "Mutation".to_string(),
            description: None,
            fields: mutation_fields,
            interfaces: Vec::new(),
        })
        .resolver("Mutation", "createUser", |ctx| {
            let name = ctx.get_arg_str("name").unwrap_or("Anonymous");
            let mut map = HashMap::new();
            map.insert("id".to_string(), GraphQLValue::String("new_99".to_string()));
            map.insert("name".to_string(), GraphQLValue::String(name.to_string()));
            Ok(GraphQLValue::Object(map))
        })
        .build();

    let engine = GraphQLEngine::new(schema);
    let mut vars = HashMap::new();
    vars.insert("uName".to_string(), GraphQLValue::String("Alice".to_string()));

    let query = r#"
        mutation AddUser($uName: String!) {
            createUser(name: $uName) {
                id
                name
            }
        }
    "#;

    let response = engine.execute(query, Some(vars), None, None);
    assert!(response.errors.is_empty());
    let json = response.to_json();
    assert!(json.contains("\"id\":\"new_99\""));
    assert!(json.contains("\"name\":\"Alice\""));
}

#[test]
fn test_introspection_query() {
    let schema = SchemaBuilder::new("Query")
        .add_object(ObjectType {
            name: "Query".to_string(),
            description: None,
            fields: HashMap::new(),
            interfaces: Vec::new(),
        })
        .build();

    let engine = GraphQLEngine::new(schema);
    let query = r#"
        query {
            __typename
            __schema {
                queryType {
                    name
                }
            }
        }
    "#;

    let response = engine.execute(query, None, None, None);
    assert!(response.errors.is_empty());
    let json = response.to_json();
    assert!(json.contains("\"__typename\":\"Query\""));
    assert!(json.contains("\"queryType\":{\"name\":\"Query\"}"));
}

#[test]
fn test_custom_scalars() {
    assert!(DateTimeScalar.parse_value(&GraphQLValue::String("2026-10-29T21:00:00Z".to_string())).is_ok());
    assert!(DateTimeScalar.parse_value(&GraphQLValue::String("invalid".to_string())).is_err());

    let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
    assert!(UuidScalar.parse_value(&GraphQLValue::String(uuid_str.to_string())).is_ok());
    assert!(UuidScalar.parse_value(&GraphQLValue::String("bad-uuid".to_string())).is_err());
}

#[test]
fn test_validation_errors() {
    let schema = SchemaBuilder::new("Query")
        .add_object(ObjectType {
            name: "Query".to_string(),
            description: None,
            fields: HashMap::new(),
            interfaces: Vec::new(),
        })
        .build();

    let engine = GraphQLEngine::new(schema);
    let query = "{ nonExistentField }";
    let response = engine.execute(query, None, None, None);
    assert!(!response.errors.is_empty());
    assert!(response.errors[0].message.contains("Field 'nonExistentField' does not exist"));
}
