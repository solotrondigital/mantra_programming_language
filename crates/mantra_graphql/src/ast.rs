use crate::types::GraphQLValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationDefinition {
    pub operation_type: OperationType,
    pub name: Option<String>,
    pub variable_definitions: Vec<VariableDefinition>,
    pub directives: Vec<Directive>,
    pub selection_set: SelectionSet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDefinition {
    pub name: String,
    pub var_type: String,
    pub default_value: Option<GraphQLValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionSet {
    pub selections: Vec<Selection>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    Field(FieldSelection),
    FragmentSpread(String),
    InlineFragment(InlineFragment),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldSelection {
    pub alias: Option<String>,
    pub name: String,
    pub arguments: Vec<Argument>,
    pub directives: Vec<Directive>,
    pub selection_set: Option<SelectionSet>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub value: ValueNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueNode {
    Variable(String),
    Literal(GraphQLValue),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentDefinition {
    pub name: String,
    pub type_condition: String,
    pub directives: Vec<Directive>,
    pub selection_set: SelectionSet,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFragment {
    pub type_condition: Option<String>,
    pub directives: Vec<Directive>,
    pub selection_set: SelectionSet,
}
