use crate::ast::*;
use crate::lexer::{Lexer, Token};
use crate::types::GraphQLValue;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn from_source(source: &str) -> Result<Document, String> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token()?;
            if tok == Token::Eof {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse_document()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            tok
        } else {
            Token::Eof
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let tok = self.advance();
        if tok == expected {
            Ok(())
        } else {
            Err(format!("Expected token {:?}, found {:?}", expected, tok))
        }
    }

    fn expect_name(&mut self) -> Result<String, String> {
        match self.advance() {
            Token::Name(s) => Ok(s),
            other => Err(format!("Expected identifier/name, found {:?}", other)),
        }
    }

    pub fn parse_document(&mut self) -> Result<Document, String> {
        let mut definitions = Vec::new();

        while self.peek() != &Token::Eof {
            match self.peek() {
                Token::LeftBrace => {
                    // Shorthand query: { field }
                    let selection_set = self.parse_selection_set()?;
                    definitions.push(Definition::Operation(OperationDefinition {
                        operation_type: OperationType::Query,
                        name: None,
                        variable_definitions: Vec::new(),
                        directives: Vec::new(),
                        selection_set,
                    }));
                }
                Token::Name(n) if n == "query" || n == "mutation" || n == "subscription" => {
                    definitions.push(Definition::Operation(self.parse_operation_definition()?));
                }
                Token::Name(n) if n == "fragment" => {
                    definitions.push(Definition::Fragment(self.parse_fragment_definition()?));
                }
                other => {
                    return Err(format!("Unexpected token at top level of GraphQL document: {:?}", other));
                }
            }
        }

        Ok(Document { definitions })
    }

    fn parse_operation_definition(&mut self) -> Result<OperationDefinition, String> {
        let op_type_str = self.expect_name()?;
        let operation_type = match op_type_str.as_str() {
            "query" => OperationType::Query,
            "mutation" => OperationType::Mutation,
            "subscription" => OperationType::Subscription,
            other => return Err(format!("Unknown operation type '{}'", other)),
        };

        let mut name = None;
        if let Token::Name(n) = self.peek() {
            if n != "fragment" && n != "query" && n != "mutation" && n != "subscription" {
                name = Some(n.clone());
                self.advance();
            }
        }

        let mut variable_definitions = Vec::new();
        if self.peek() == &Token::LeftParen {
            self.advance();
            while self.peek() != &Token::RightParen && self.peek() != &Token::Eof {
                variable_definitions.push(self.parse_variable_definition()?);
            }
            self.expect(Token::RightParen)?;
        }

        let directives = self.parse_directives()?;
        let selection_set = self.parse_selection_set()?;

        Ok(OperationDefinition {
            operation_type,
            name,
            variable_definitions,
            directives,
            selection_set,
        })
    }

    fn parse_variable_definition(&mut self) -> Result<VariableDefinition, String> {
        self.expect(Token::Dollar)?;
        let name = self.expect_name()?;
        self.expect(Token::Colon)?;
        let var_type = self.parse_type_signature()?;
        let mut default_value = None;
        if self.peek() == &Token::Equals {
            self.advance();
            if let ValueNode::Literal(lit) = self.parse_value_node()? {
                default_value = Some(lit);
            }
        }
        Ok(VariableDefinition {
            name,
            var_type,
            default_value,
        })
    }

    fn parse_type_signature(&mut self) -> Result<String, String> {
        let mut s = if self.peek() == &Token::LeftBracket {
            self.advance();
            let inner = self.parse_type_signature()?;
            self.expect(Token::RightBracket)?;
            format!("[{}]", inner)
        } else {
            self.expect_name()?
        };
        if self.peek() == &Token::Exclamation {
            self.advance();
            s.push('!');
        }
        Ok(s)
    }

    fn parse_selection_set(&mut self) -> Result<SelectionSet, String> {
        self.expect(Token::LeftBrace)?;
        let mut selections = Vec::new();
        while self.peek() != &Token::RightBrace && self.peek() != &Token::Eof {
            selections.push(self.parse_selection()?);
        }
        self.expect(Token::RightBrace)?;
        Ok(SelectionSet { selections })
    }

    fn parse_selection(&mut self) -> Result<Selection, String> {
        if self.peek() == &Token::Ellipsis {
            self.advance();
            if let Token::Name(n) = self.peek() {
                if n == "on" {
                    // Inline fragment: ... on Type { ... }
                    self.advance();
                    let type_cond = self.expect_name()?;
                    let directives = self.parse_directives()?;
                    let selection_set = self.parse_selection_set()?;
                    return Ok(Selection::InlineFragment(InlineFragment {
                        type_condition: Some(type_cond),
                        directives,
                        selection_set,
                    }));
                }
            }
            if self.peek() == &Token::LeftBrace {
                // Untyped inline fragment: ... { ... }
                let selection_set = self.parse_selection_set()?;
                return Ok(Selection::InlineFragment(InlineFragment {
                    type_condition: None,
                    directives: Vec::new(),
                    selection_set,
                }));
            }
            // Named fragment spread: ... FragmentName
            let frag_name = self.expect_name()?;
            return Ok(Selection::FragmentSpread(frag_name));
        }

        // Field selection: [alias:] name [(args)] [@directives] [{ selection_set }]
        let first_name = self.expect_name()?;
        let (alias, name) = if self.peek() == &Token::Colon {
            self.advance();
            let actual_name = self.expect_name()?;
            (Some(first_name), actual_name)
        } else {
            (None, first_name)
        };

        let mut arguments = Vec::new();
        if self.peek() == &Token::LeftParen {
            self.advance();
            while self.peek() != &Token::RightParen && self.peek() != &Token::Eof {
                let arg_name = self.expect_name()?;
                self.expect(Token::Colon)?;
                let arg_val = self.parse_value_node()?;
                arguments.push(Argument {
                    name: arg_name,
                    value: arg_val,
                });
            }
            self.expect(Token::RightParen)?;
        }

        let directives = self.parse_directives()?;

        let selection_set = if self.peek() == &Token::LeftBrace {
            Some(self.parse_selection_set()?)
        } else {
            None
        };

        Ok(Selection::Field(FieldSelection {
            alias,
            name,
            arguments,
            directives,
            selection_set,
        }))
    }

    fn parse_directives(&mut self) -> Result<Vec<Directive>, String> {
        let mut directives = Vec::new();
        while self.peek() == &Token::At {
            self.advance();
            let name = self.expect_name()?;
            let mut arguments = Vec::new();
            if self.peek() == &Token::LeftParen {
                self.advance();
                while self.peek() != &Token::RightParen && self.peek() != &Token::Eof {
                    let arg_name = self.expect_name()?;
                    self.expect(Token::Colon)?;
                    let arg_val = self.parse_value_node()?;
                    arguments.push(Argument {
                        name: arg_name,
                        value: arg_val,
                    });
                }
                self.expect(Token::RightParen)?;
            }
            directives.push(Directive { name, arguments });
        }
        Ok(directives)
    }

    fn parse_fragment_definition(&mut self) -> Result<FragmentDefinition, String> {
        self.expect_name()?; // 'fragment'
        let name = self.expect_name()?;
        let on_tok = self.expect_name()?;
        if on_tok != "on" {
            return Err(format!("Expected 'on' in fragment definition, found '{}'", on_tok));
        }
        let type_condition = self.expect_name()?;
        let directives = self.parse_directives()?;
        let selection_set = self.parse_selection_set()?;
        Ok(FragmentDefinition {
            name,
            type_condition,
            directives,
            selection_set,
        })
    }

    fn parse_value_node(&mut self) -> Result<ValueNode, String> {
        if self.peek() == &Token::Dollar {
            self.advance();
            let var_name = self.expect_name()?;
            return Ok(ValueNode::Variable(var_name));
        }

        let lit = self.parse_literal_value()?;
        Ok(ValueNode::Literal(lit))
    }

    fn parse_literal_value(&mut self) -> Result<GraphQLValue, String> {
        match self.peek() {
            Token::Int(i) => {
                let v = *i;
                self.advance();
                Ok(GraphQLValue::Int(v))
            }
            Token::Float(f) => {
                let v = *f;
                self.advance();
                Ok(GraphQLValue::Float(v))
            }
            Token::String(s) => {
                let v = s.clone();
                self.advance();
                Ok(GraphQLValue::String(v))
            }
            Token::Name(n) => {
                let name = n.clone();
                self.advance();
                match name.as_str() {
                    "true" => Ok(GraphQLValue::Boolean(true)),
                    "false" => Ok(GraphQLValue::Boolean(false)),
                    "null" => Ok(GraphQLValue::Null),
                    _ => Ok(GraphQLValue::Enum(name)),
                }
            }
            Token::LeftBracket => {
                self.advance();
                let mut list = Vec::new();
                while self.peek() != &Token::RightBracket && self.peek() != &Token::Eof {
                    list.push(self.parse_literal_value()?);
                }
                self.expect(Token::RightBracket)?;
                Ok(GraphQLValue::List(list))
            }
            Token::LeftBrace => {
                self.advance();
                let mut map = HashMap::new();
                while self.peek() != &Token::RightBrace && self.peek() != &Token::Eof {
                    let field_name = self.expect_name()?;
                    self.expect(Token::Colon)?;
                    let field_val = self.parse_literal_value()?;
                    map.insert(field_name, field_val);
                }
                self.expect(Token::RightBrace)?;
                Ok(GraphQLValue::Object(map))
            }
            other => Err(format!("Unexpected token when parsing GraphQL value: {:?}", other)),
        }
    }
}
