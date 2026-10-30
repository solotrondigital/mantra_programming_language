use mantrac_ast::*;
use mantrac_lexer::span::Span;
use mantrac_lexer::token::{Token, TokenKind};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub span: Span,
    pub code: &'static str,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] at {}:{}: {}", self.code, self.span.line, self.span.col, self.message)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut declarations = Vec::new();
        self.skip_newlines();

        while !self.is_eof() {
            let decl = self.parse_declaration()?;
            declarations.push(decl);
            self.skip_newlines();
        }

        Ok(Program { declarations })
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        self.skip_newlines();

        // Check for decorators
        let mut decorator = None;
        if self.check(&TokenKind::At) {
            self.advance();
            let name = self.consume_ident("Expected decorator name")?;
            let mut full_dec = name;
            while self.match_token(&TokenKind::Dot) {
                let part = self.consume_ident("Expected decorator attribute")?;
                full_dec.push('.');
                full_dec.push_str(&part);
            }
            decorator = Some(full_dec);
            self.consume_newline_or_eof()?;
            self.skip_newlines();
        }

        let mut is_abstract = false;
        let mut is_override = false;
        let mut is_async = false;
        let mut visibility = Visibility::Default;

        // Parse modifiers: pub, private, abstract, override, async
        loop {
            if self.match_token(&TokenKind::Pub) {
                visibility = Visibility::Pub;
            } else if self.match_token(&TokenKind::Private) {
                visibility = Visibility::Private;
            } else if self.match_token(&TokenKind::Protected) {
                visibility = Visibility::Protected;
            } else if self.match_token(&TokenKind::Abstract) {
                is_abstract = true;
            } else if self.match_token(&TokenKind::Override) {
                is_override = true;
            } else if self.match_token(&TokenKind::Async) {
                is_async = true;
            } else {
                break;
            }
        }

        if self.match_token(&TokenKind::Module) {
            let start_span = self.previous().span;
            let name = self.consume_ident("Expected module name")?;
            self.consume_newline_or_eof()?;
            return Ok(Declaration::Module(name, start_span));
        }

        if self.match_token(&TokenKind::Import) {
            let start_span = self.previous().span;
            let mut parts = vec![self.consume_ident("Expected package/module name to import")?];
            while self.match_token(&TokenKind::Dot) {
                if self.match_token(&TokenKind::LBrace) {
                    while !self.check(&TokenKind::RBrace) && !self.is_eof() {
                        let symbol = self.consume_ident("Expected symbol name inside import list")?;
                        parts.push(symbol);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.consume(&TokenKind::RBrace, "Expected '}' after import list")?;
                    break;
                } else if self.match_token(&TokenKind::Star) {
                    parts.push("*".to_string());
                    break;
                } else {
                    parts.push(self.consume_ident("Expected identifier in import path")?);
                }
            }
            let alias = if self.match_token(&TokenKind::As) {
                Some(self.consume_ident("Expected alias name after 'as'")?)
            } else {
                None
            };
            self.consume_newline_or_eof()?;
            return Ok(Declaration::Import {
                module: parts,
                alias,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::From) {
            let start_span = self.previous().span;
            let mut parts = vec![self.consume_ident("Expected module name after 'from'")?];
            while self.match_token(&TokenKind::Dot) {
                parts.push(self.consume_ident("Expected identifier in module path")?);
            }
            self.consume(&TokenKind::Import, "Expected 'import' after module path in from-import")?;
            if self.match_token(&TokenKind::Star) {
                parts.push("*".to_string());
            } else if self.match_token(&TokenKind::LBrace) {
                while !self.check(&TokenKind::RBrace) && !self.is_eof() {
                    let symbol = self.consume_ident("Expected symbol name in import list")?;
                    parts.push(symbol);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(&TokenKind::RBrace, "Expected '}' after import list")?;
            } else {
                let first = self.consume_ident("Expected symbol to import")?;
                parts.push(first);
                while self.match_token(&TokenKind::Comma) {
                    let next = self.consume_ident("Expected symbol to import after comma")?;
                    parts.push(next);
                }
            }
            let alias = if self.match_token(&TokenKind::As) {
                Some(self.consume_ident("Expected alias name after 'as'")?)
            } else {
                None
            };
            self.consume_newline_or_eof()?;
            return Ok(Declaration::Import {
                module: parts,
                alias,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::Class) {
            return Ok(Declaration::Class(self.parse_class(visibility, is_abstract)?));
        }

        if self.match_token(&TokenKind::Interface) {
            return Ok(Declaration::Interface(self.parse_interface()?));
        }

        if self.match_token(&TokenKind::Def) {
            let func = self.parse_function(visibility, is_override, false, is_abstract, is_async, decorator)?;
            return Ok(Declaration::Function(func));
        }

        if self.match_token(&TokenKind::Let) {
            let start_span = self.previous().span;
            let name = self.consume_ident("Expected variable name after 'let'")?;
            let ty = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.consume(&TokenKind::Eq, "Expected '=' in variable declaration")?;
            let init = self.parse_expr()?;
            self.consume_newline_or_eof()?;
            return Ok(Declaration::Var(VarDecl {
                name,
                ty,
                is_mut: false,
                init,
                span: start_span,
            }));
        }

        if self.match_token(&TokenKind::Mut) {
            let start_span = self.previous().span;
            let name = self.consume_ident("Expected variable name after 'mut'")?;
            let ty = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.consume(&TokenKind::Eq, "Expected '=' in variable declaration")?;
            let init = self.parse_expr()?;
            self.consume_newline_or_eof()?;
            return Ok(Declaration::Var(VarDecl {
                name,
                ty,
                is_mut: true,
                init,
                span: start_span,
            }));
        }

        // Top level statement or typed variable declaration e.g. name: String = "Mantra"
        if let TokenKind::Ident(ref name) = self.peek().kind {
            if self.peek_ahead(1).kind == TokenKind::Colon {
                let name = name.clone();
                let start_span = self.peek().span;
                self.advance(); // consume ident
                self.advance(); // consume colon
                let ty = self.parse_type()?;
                if self.match_token(&TokenKind::Eq) {
                    let init = self.parse_expr()?;
                    self.consume_newline_or_eof()?;
                    return Ok(Declaration::Var(VarDecl {
                        name,
                        ty: Some(ty),
                        is_mut: false,
                        init,
                        span: start_span,
                    }));
                }
            }
        }

        let stmt = self.parse_stmt()?;
        Ok(Declaration::Stmt(stmt))
    }

    fn parse_class(&mut self, visibility: Visibility, is_abstract: bool) -> Result<ClassDecl, ParserError> {
        let start_span = self.previous().span;
        let name = self.consume_ident("Expected class name")?;

        let type_params = if self.match_token(&TokenKind::Lt) {
            self.parse_type_param_list()?
        } else {
            Vec::new()
        };

        let base_class = if self.match_token(&TokenKind::Extends) {
            Some(self.consume_ident("Expected base class name after 'extends'")?)
        } else {
            None
        };

        let mut interfaces = Vec::new();
        if self.match_token(&TokenKind::Implements) {
            loop {
                interfaces.push(self.consume_ident("Expected interface name after 'implements'")?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }

        self.consume(&TokenKind::Colon, "Expected ':' after class declaration")?;
        self.consume_newline_or_eof()?;

        let mut members = Vec::new();
        self.consume(&TokenKind::Indent, "Expected indented block for class body")?;

        while !self.check(&TokenKind::Dedent) && !self.is_eof() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) || self.is_eof() {
                break;
            }

            let member = self.parse_class_member()?;
            members.push(member);
            self.skip_newlines();
        }

        self.consume(&TokenKind::Dedent, "Expected end of class body")?;

        Ok(ClassDecl {
            name,
            type_params,
            base_class,
            interfaces,
            is_abstract,
            visibility,
            members,
            span: start_span,
        })
    }

    fn parse_class_member(&mut self) -> Result<ClassMember, ParserError> {
        let mut decorator = None;
        if self.check(&TokenKind::At) {
            self.advance();
            let dec_name = self.consume_ident("Expected decorator name")?;
            let mut full_dec = dec_name;
            while self.match_token(&TokenKind::Dot) {
                let part = self.consume_ident("Expected decorator attribute")?;
                full_dec.push('.');
                full_dec.push_str(&part);
            }
            decorator = Some(full_dec);
            self.consume_newline_or_eof()?;
            self.skip_newlines();
        }

        let mut is_override = false;
        let mut is_static = false;
        let mut is_abstract = false;
        let mut is_async = false;
        let mut visibility = Visibility::Default;

        loop {
            if self.match_token(&TokenKind::Pub) {
                visibility = Visibility::Pub;
            } else if self.match_token(&TokenKind::Private) {
                visibility = Visibility::Private;
            } else if self.match_token(&TokenKind::Protected) {
                visibility = Visibility::Protected;
            } else if self.match_token(&TokenKind::Static) {
                is_static = true;
            } else if self.match_token(&TokenKind::Override) {
                is_override = true;
            } else if self.match_token(&TokenKind::Abstract) {
                is_abstract = true;
            } else if self.match_token(&TokenKind::Async) {
                is_async = true;
            } else {
                break;
            }
        }

        if self.match_token(&TokenKind::Def) {
            let func = self.parse_function(visibility, is_override, is_static, is_abstract, is_async, decorator.clone())?;
            if let Some(dec) = decorator {
                if dec == "property" {
                    return Ok(ClassMember::Property(PropertyDecl {
                        name: func.name.clone(),
                        getter: func,
                        setter: None,
                        span: self.previous().span,
                    }));
                }
            }
            if is_static {
                return Ok(ClassMember::StaticMethod(func));
            } else {
                return Ok(ClassMember::Method(func));
            }
        }

        // Static field or instance field
        let member_span = self.peek().span;
        let name = self.consume_ident("Expected field or method declaration in class")?;
        let ty = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let default_val = if self.match_token(&TokenKind::Eq) {
            Some(self.parse_expr()?)
        } else {
            None
        };

        self.consume_newline_or_eof()?;

        if is_static {
            let default = default_val.ok_or_else(|| ParserError {
                message: format!("Static field '{}' must have an initial value", name),
                span: member_span,
                code: "E0011",
            })?;
            Ok(ClassMember::StaticField(StaticFieldDecl {
                name,
                ty,
                default,
                visibility,
                span: member_span,
            }))
        } else {
            Ok(ClassMember::Field(FieldDecl {
                name,
                ty,
                default: default_val,
                visibility,
                span: member_span,
            }))
        }
    }

    fn parse_interface(&mut self) -> Result<InterfaceDecl, ParserError> {
        let start_span = self.previous().span;
        let name = self.consume_ident("Expected interface name")?;

        let type_params = if self.match_token(&TokenKind::Lt) {
            self.parse_type_param_list()?
        } else {
            Vec::new()
        };

        self.consume(&TokenKind::Colon, "Expected ':' after interface declaration")?;
        self.consume_newline_or_eof()?;

        let mut methods = Vec::new();
        self.consume(&TokenKind::Indent, "Expected indented block for interface body")?;

        while !self.check(&TokenKind::Dedent) && !self.is_eof() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) || self.is_eof() {
                break;
            }

            let mut is_async = false;
            if self.match_token(&TokenKind::Async) {
                is_async = true;
            }
            self.consume(&TokenKind::Def, "Expected 'def' in interface method specification")?;
            let m_name = self.consume_ident("Expected method name")?;
            self.consume(&TokenKind::LParen, "Expected '(' after method name")?;
            let params = self.parse_param_list()?;
            self.consume(&TokenKind::RParen, "Expected ')' after parameters")?;

            let return_type = if self.match_token(&TokenKind::Arrow) {
                self.parse_type()?
            } else {
                Type::Unit
            };

            self.consume_newline_or_eof()?;
            methods.push(MethodSignature {
                name: m_name,
                params,
                return_type,
                is_async,
                span: self.previous().span,
            });
            self.skip_newlines();
        }

        self.consume(&TokenKind::Dedent, "Expected end of interface body")?;

        Ok(InterfaceDecl {
            name,
            type_params,
            methods,
            span: start_span,
        })
    }

    fn parse_function(
        &mut self,
        visibility: Visibility,
        is_override: bool,
        is_static: bool,
        is_abstract: bool,
        is_async: bool,
        decorator: Option<String>,
    ) -> Result<FunctionDecl, ParserError> {
        let start_span = self.previous().span;
        let name = self.consume_ident_or_dunder("Expected function or method name")?;

        let type_params = if self.match_token(&TokenKind::Lt) {
            self.parse_type_param_list()?
        } else {
            Vec::new()
        };

        self.consume(&TokenKind::LParen, "Expected '(' after function name")?;
        let params = self.parse_param_list()?;
        self.consume(&TokenKind::RParen, "Expected ')' after parameter list")?;

        let return_type = if self.match_token(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        if is_abstract {
            self.consume_newline_or_eof()?;
            return Ok(FunctionDecl {
                name,
                type_params,
                params,
                return_type,
                body: Vec::new(),
                visibility,
                is_override,
                is_static,
                is_abstract: true,
                is_async,
                decorator,
                span: start_span,
            });
        }

        self.consume(&TokenKind::Colon, "Expected ':' after function signature")?;
        self.consume_newline_or_eof()?;

        let body = self.parse_block()?;

        Ok(FunctionDecl {
            name,
            type_params,
            params,
            return_type,
            body,
            visibility,
            is_override,
            is_static,
            is_abstract: false,
            is_async,
            decorator,
            span: start_span,
        })
    }

    fn parse_param_list(&mut self) -> Result<Vec<Param>, ParserError> {
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.is_eof() {
            let start_span = self.peek().span;
            let name = if self.match_token(&TokenKind::SelfKw) {
                "self".to_string()
            } else {
                self.consume_ident("Expected parameter name")?
            };

            let ty = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };

            let default_val = if self.match_token(&TokenKind::Eq) {
                Some(self.parse_expr()?)
            } else {
                None
            };

            params.push(Param {
                name,
                ty,
                default_val,
                span: start_span,
            });

            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        self.skip_newlines();
        self.consume(&TokenKind::Indent, "Expected indented block")?;
        let mut statements = Vec::new();

        while !self.check(&TokenKind::Dedent) && !self.is_eof() {
            self.skip_newlines();
            if self.check(&TokenKind::Dedent) || self.is_eof() {
                break;
            }
            let stmt = self.parse_stmt()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        self.consume(&TokenKind::Dedent, "Expected end of block")?;
        Ok(statements)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        self.skip_newlines();

        if self.match_token(&TokenKind::Let) {
            let start_span = self.previous().span;
            let name = self.consume_ident("Expected variable name after 'let'")?;
            let ty = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.consume(&TokenKind::Eq, "Expected '=' in variable binding")?;
            let init = self.parse_expr()?;
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Let {
                name,
                ty,
                init,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::Mut) {
            let start_span = self.previous().span;
            let name = self.consume_ident("Expected variable name after 'mut'")?;
            let ty = if self.match_token(&TokenKind::Colon) {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.consume(&TokenKind::Eq, "Expected '=' in variable binding")?;
            let init = self.parse_expr()?;
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Mut {
                name,
                ty,
                init,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::If) {
            let start_span = self.previous().span;
            let condition = self.parse_expr()?;
            self.consume(&TokenKind::Colon, "Expected ':' after if condition")?;
            self.consume_newline_or_eof()?;
            let then_branch = self.parse_block()?;

            let mut elif_branches = Vec::new();
            self.skip_newlines();
            while self.match_token(&TokenKind::Elif) {
                let elif_cond = self.parse_expr()?;
                self.consume(&TokenKind::Colon, "Expected ':' after elif condition")?;
                self.consume_newline_or_eof()?;
                let elif_body = self.parse_block()?;
                elif_branches.push((elif_cond, elif_body));
                self.skip_newlines();
            }

            let else_branch = if self.match_token(&TokenKind::Else) {
                self.consume(&TokenKind::Colon, "Expected ':' after 'else'")?;
                self.consume_newline_or_eof()?;
                Some(self.parse_block()?)
            } else {
                None
            };

            return Ok(Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::While) {
            let start_span = self.previous().span;
            let condition = self.parse_expr()?;
            self.consume(&TokenKind::Colon, "Expected ':' after while condition")?;
            self.consume_newline_or_eof()?;
            let body = self.parse_block()?;
            return Ok(Stmt::While {
                condition,
                body,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::For) {
            let start_span = self.previous().span;
            let var = self.consume_ident("Expected loop variable name after 'for'")?;
            self.consume(&TokenKind::In, "Expected 'in' after for variable")?;
            let iterable = self.parse_expr()?;
            self.consume(&TokenKind::Colon, "Expected ':' after for loop iterable")?;
            self.consume_newline_or_eof()?;
            let body = self.parse_block()?;
            return Ok(Stmt::For {
                var,
                iterable,
                body,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::Return) {
            let start_span = self.previous().span;
            let value = if self.check(&TokenKind::Newline) || self.check(&TokenKind::Eof) || self.check(&TokenKind::Dedent) {
                None
            } else {
                Some(self.parse_expr()?)
            };
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Return {
                value,
                span: start_span,
            });
        }

        if self.match_token(&TokenKind::Break) {
            let span = self.previous().span;
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Break(span));
        }

        if self.match_token(&TokenKind::Continue) {
            let span = self.previous().span;
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Continue(span));
        }

        // Could be a typed variable declaration e.g. "count: Int = 0"
        if let TokenKind::Ident(ref id) = self.peek().kind {
            if self.peek_ahead(1).kind == TokenKind::Colon {
                let name = id.clone();
                let start_span = self.peek().span;
                self.advance(); // ident
                self.advance(); // colon
                let ty = self.parse_type()?;
                if self.match_token(&TokenKind::Eq) {
                    let init = self.parse_expr()?;
                    self.consume_newline_or_eof()?;
                    return Ok(Stmt::Let {
                        name,
                        ty: Some(ty),
                        init,
                        span: start_span,
                    });
                }
            }
        }

        // Parse expression or assignment
        let expr = self.parse_expr()?;

        // Optional type annotation in assignment: target: Type = value
        if self.match_token(&TokenKind::Colon) {
            let _ann_ty = self.parse_type()?;
            if self.match_token(&TokenKind::Eq) {
                let value = self.parse_expr()?;
                let span = expr.span().merge(value.span());
                self.consume_newline_or_eof()?;
                return Ok(Stmt::Assign {
                    target: expr,
                    value,
                    span,
                });
            } else {
                self.consume_newline_or_eof()?;
                return Ok(Stmt::Expr(expr));
            }
        }

        if self.match_token(&TokenKind::Eq) {
            let value = self.parse_expr()?;
            let span = expr.span().merge(value.span());
            self.consume_newline_or_eof()?;
            return Ok(Stmt::Assign {
                target: expr,
                value,
                span,
            });
        }

        if self.match_token(&TokenKind::PlusEq) {
            let value = self.parse_expr()?;
            let span = expr.span().merge(value.span());
            self.consume_newline_or_eof()?;
            return Ok(Stmt::AugAssign {
                target: expr,
                op: BinaryOp::Add,
                value,
                span,
            });
        }

        if self.match_token(&TokenKind::MinusEq) {
            let value = self.parse_expr()?;
            let span = expr.span().merge(value.span());
            self.consume_newline_or_eof()?;
            return Ok(Stmt::AugAssign {
                target: expr,
                op: BinaryOp::Sub,
                value,
                span,
            });
        }

        if self.match_token(&TokenKind::StarEq) {
            let value = self.parse_expr()?;
            let span = expr.span().merge(value.span());
            self.consume_newline_or_eof()?;
            return Ok(Stmt::AugAssign {
                target: expr,
                op: BinaryOp::Mul,
                value,
                span,
            });
        }

        if self.match_token(&TokenKind::SlashEq) {
            let value = self.parse_expr()?;
            let span = expr.span().merge(value.span());
            self.consume_newline_or_eof()?;
            return Ok(Stmt::AugAssign {
                target: expr,
                op: BinaryOp::Div,
                value,
                span,
            });
        }

        self.consume_newline_or_eof()?;
        Ok(Stmt::Expr(expr))
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_logical_and()?;
        while self.match_token(&TokenKind::Or) {
            let right = self.parse_logical_and()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_equality()?;
        while self.match_token(&TokenKind::And) {
            let right = self.parse_equality()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_comparison()?;
        while self.check(&TokenKind::EqEq) || self.check(&TokenKind::NotEq) {
            let op = if self.match_token(&TokenKind::EqEq) {
                BinaryOp::Eq
            } else {
                self.advance();
                BinaryOp::NotEq
            };
            let right = self.parse_comparison()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_term()?;
        while self.check(&TokenKind::Lt)
            || self.check(&TokenKind::LtEq)
            || self.check(&TokenKind::Gt)
            || self.check(&TokenKind::GtEq)
        {
            let op = if self.match_token(&TokenKind::Lt) {
                BinaryOp::Lt
            } else if self.match_token(&TokenKind::LtEq) {
                BinaryOp::LtEq
            } else if self.match_token(&TokenKind::Gt) {
                BinaryOp::Gt
            } else {
                self.advance();
                BinaryOp::GtEq
            };
            let right = self.parse_term()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_factor()?;
        while self.check(&TokenKind::Plus) || self.check(&TokenKind::Minus) {
            let op = if self.match_token(&TokenKind::Plus) {
                BinaryOp::Add
            } else {
                self.advance();
                BinaryOp::Sub
            };
            let right = self.parse_factor()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_unary()?;
        while self.check(&TokenKind::Star) || self.check(&TokenKind::Slash) || self.check(&TokenKind::Percent) {
            let op = if self.match_token(&TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_token(&TokenKind::Slash) {
                BinaryOp::Div
            } else {
                self.advance();
                BinaryOp::Mod
            };
            let right = self.parse_unary()?;
            let span = left.span().merge(right.span());
            left = Expr::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&TokenKind::Await) {
            let op_span = self.previous().span;
            let operand = self.parse_unary()?;
            let span = op_span.merge(operand.span());
            return Ok(Expr::Await {
                value: Box::new(operand),
                span,
            });
        }
        if self.match_token(&TokenKind::Not) {
            let op_span = self.previous().span;
            let operand = self.parse_unary()?;
            let span = op_span.merge(operand.span());
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
                span,
            });
        }
        if self.match_token(&TokenKind::Minus) {
            let op_span = self.previous().span;
            let operand = self.parse_unary()?;
            let span = op_span.merge(operand.span());
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
                span,
            });
        }
        self.parse_call_and_postfix()
    }

    fn parse_call_and_postfix(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&TokenKind::LParen) {
                let args = self.parse_arg_list()?;
                self.consume(&TokenKind::RParen, "Expected ')' after function arguments")?;
                let span = expr.span().merge(self.previous().span);
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span,
                };
            } else if self.match_token(&TokenKind::Dot) {
                let member_name = self.consume_ident_or_dunder("Expected member name after '.'")?;
                if self.match_token(&TokenKind::LParen) {
                    let args = self.parse_arg_list()?;
                    self.consume(&TokenKind::RParen, "Expected ')' after method arguments")?;
                    let span = expr.span().merge(self.previous().span);
                    expr = Expr::MethodCall {
                        receiver: Box::new(expr),
                        method: member_name,
                        args,
                        span,
                    };
                } else {
                    let span = expr.span().merge(self.previous().span);
                    expr = Expr::FieldAccess {
                        receiver: Box::new(expr),
                        field: member_name,
                        span,
                    };
                }
            } else if self.match_token(&TokenKind::LBracket) {
                let index = self.parse_expr()?;
                self.consume(&TokenKind::RBracket, "Expected ']' after index")?;
                let span = expr.span().merge(self.previous().span);
                expr = Expr::Index {
                    receiver: Box::new(expr),
                    index: Box::new(index),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_arg_list(&mut self) -> Result<Vec<Expr>, ParserError> {
        let mut args = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.is_eof() {
            args.push(self.parse_expr()?);
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        let tok = self.peek().clone();

        match tok.kind {
            TokenKind::Integer(i) => {
                self.advance();
                Ok(Expr::Literal(Literal::Integer(i), tok.span))
            }
            TokenKind::Float(f) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(f), tok.span))
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s), tok.span))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true), tok.span))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false), tok.span))
            }
            TokenKind::None => {
                self.advance();
                Ok(Expr::Literal(Literal::None, tok.span))
            }
            TokenKind::SelfKw => {
                self.advance();
                Ok(Expr::Ident("self".to_string(), tok.span))
            }
            TokenKind::Super => {
                self.advance();
                self.consume(&TokenKind::LParen, "Expected '()' after super")?;
                self.consume(&TokenKind::RParen, "Expected '()' after super")?;
                self.consume(&TokenKind::Dot, "Expected '.' after super()")?;
                let method = self.consume_ident_or_dunder("Expected method name after super().")?;
                self.consume(&TokenKind::LParen, "Expected '(' after super method")?;
                let args = self.parse_arg_list()?;
                self.consume(&TokenKind::RParen, "Expected ')' after super call arguments")?;
                let span = tok.span.merge(self.previous().span);
                if method == "__init__" {
                    Ok(Expr::SuperInitCall { args, span })
                } else {
                    Ok(Expr::SuperMethodCall { method, args, span })
                }
            }
            TokenKind::Ident(id) => {
                self.advance();
                Ok(Expr::Ident(id, tok.span))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(&TokenKind::RParen, "Expected ')' after parenthesized expression")?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.is_eof() {
                    items.push(self.parse_expr()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(&TokenKind::RBracket, "Expected ']' at end of list literal")?;
                let span = tok.span.merge(self.previous().span);
                Ok(Expr::List(items, span))
            }
            TokenKind::LBrace => {
                self.advance();
                let mut entries = Vec::new();
                while !self.check(&TokenKind::RBrace) && !self.is_eof() {
                    let k = self.parse_expr()?;
                    self.consume(&TokenKind::Colon, "Expected ':' after map key")?;
                    let v = self.parse_expr()?;
                    entries.push((k, v));
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.consume(&TokenKind::RBrace, "Expected '}' at end of map literal")?;
                let span = tok.span.merge(self.previous().span);
                Ok(Expr::Map(entries, span))
            }
            _ => Err(ParserError {
                message: format!("Unexpected token: '{}'", tok.kind),
                span: tok.span,
                code: "E0020",
            }),
        }
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let name = self.consume_ident("Expected type name")?;
        match name.as_str() {
            "Int" => Ok(Type::Int),
            "Float" => Ok(Type::Float),
            "Bool" => Ok(Type::Bool),
            "String" => Ok(Type::String),
            "Byte" => Ok(Type::Byte),
            "Unit" => Ok(Type::Unit),
            "None" => Ok(Type::None),
            "Any" => Ok(Type::Any),
            "Option" => {
                let closing = if self.match_token(&TokenKind::Lt) {
                    Some(TokenKind::Gt)
                } else if self.match_token(&TokenKind::LBracket) {
                    Some(TokenKind::RBracket)
                } else {
                    None
                };
                if let Some(close_tok) = closing {
                    let inner = self.parse_type()?;
                    self.consume(&close_tok, "Expected closing bracket after Option type parameter")?;
                    Ok(Type::Option(Box::new(inner)))
                } else {
                    Ok(Type::Option(Box::new(Type::Any)))
                }
            }
            "Result" => {
                let closing = if self.match_token(&TokenKind::Lt) {
                    Some(TokenKind::Gt)
                } else if self.match_token(&TokenKind::LBracket) {
                    Some(TokenKind::RBracket)
                } else {
                    None
                };
                if let Some(close_tok) = closing {
                    let ok = self.parse_type()?;
                    self.consume(&TokenKind::Comma, "Expected ',' in Result type")?;
                    let err = self.parse_type()?;
                    self.consume(&close_tok, "Expected closing bracket after Result type parameters")?;
                    Ok(Type::Result(Box::new(ok), Box::new(err)))
                } else {
                    Ok(Type::Result(Box::new(Type::Any), Box::new(Type::Any)))
                }
            }
            "List" => {
                if self.match_token(&TokenKind::Lt) {
                    let elem = self.parse_type()?;
                    self.consume(&TokenKind::Gt, "Expected '>' after List type parameter")?;
                    Ok(Type::List(Box::new(elem)))
                } else if self.match_token(&TokenKind::LBracket) {
                    let elem = self.parse_type()?;
                    self.consume(&TokenKind::RBracket, "Expected ']' after List type parameter")?;
                    Ok(Type::List(Box::new(elem)))
                } else {
                    Ok(Type::List(Box::new(Type::Any)))
                }
            }
            "Map" => {
                let closing = if self.match_token(&TokenKind::Lt) {
                    Some(TokenKind::Gt)
                } else if self.match_token(&TokenKind::LBracket) {
                    Some(TokenKind::RBracket)
                } else {
                    None
                };
                if let Some(close_tok) = closing {
                    let k = self.parse_type()?;
                    self.consume(&TokenKind::Comma, "Expected ',' in Map type")?;
                    let v = self.parse_type()?;
                    self.consume(&close_tok, "Expected closing bracket after Map type parameters")?;
                    Ok(Type::Map(Box::new(k), Box::new(v)))
                } else {
                    Ok(Type::Map(Box::new(Type::Any), Box::new(Type::Any)))
                }
            }
            custom => {
                let mut type_args = Vec::new();
                let closing = if self.match_token(&TokenKind::Lt) {
                    Some(TokenKind::Gt)
                } else if self.match_token(&TokenKind::LBracket) {
                    Some(TokenKind::RBracket)
                } else {
                    None
                };
                if let Some(close_tok) = closing {
                    loop {
                        type_args.push(self.parse_type()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.consume(&close_tok, "Expected closing bracket after type parameters")?;
                }
                Ok(Type::Custom(custom.to_string(), type_args))
            }
        }
    }

    fn parse_type_param_list(&mut self) -> Result<Vec<String>, ParserError> {
        let mut params = Vec::new();
        while !self.check(&TokenKind::Gt) && !self.is_eof() {
            params.push(self.consume_ident("Expected type parameter name")?);
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.consume(&TokenKind::Gt, "Expected '>' after type parameter list")?;
        Ok(params)
    }

    fn consume_ident(&mut self, err_msg: &'static str) -> Result<String, ParserError> {
        if let TokenKind::Ident(ref name) = self.peek().kind {
            let res = name.clone();
            self.advance();
            Ok(res)
        } else {
            Err(ParserError {
                message: format!("{}. Found '{}'", err_msg, self.peek().kind),
                span: self.peek().span,
                code: "E0021",
            })
        }
    }

    fn consume_ident_or_dunder(&mut self, err_msg: &'static str) -> Result<String, ParserError> {
        if let TokenKind::Ident(ref name) = self.peek().kind {
            let res = name.clone();
            self.advance();
            Ok(res)
        } else {
            Err(ParserError {
                message: format!("{}. Found '{}'", err_msg, self.peek().kind),
                span: self.peek().span,
                code: "E0021",
            })
        }
    }

    fn consume(&mut self, expected: &TokenKind, err_msg: &'static str) -> Result<(), ParserError> {
        if self.check(expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError {
                message: format!("{}. Found '{}'", err_msg, self.peek().kind),
                span: self.peek().span,
                code: "E0022",
            })
        }
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_eof() {
            kind == &TokenKind::Eof
        } else {
            // Note: ignore variant payloads for Ident/Int/etc if checking literal kind
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.cursor += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.cursor]
    }

    fn peek_ahead(&self, offset: usize) -> &Token {
        if self.cursor + offset < self.tokens.len() {
            &self.tokens[self.cursor + offset]
        } else {
            &self.tokens[self.tokens.len() - 1]
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.cursor.saturating_sub(1)]
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }

    fn skip_newlines(&mut self) {
        while !self.is_eof() && self.peek().kind == TokenKind::Newline {
            self.advance();
        }
    }

    fn consume_newline_or_eof(&mut self) -> Result<(), ParserError> {
        if self.is_eof() {
            return Ok(());
        }
        if self.match_token(&TokenKind::Newline) {
            self.skip_newlines();
            return Ok(());
        }
        if self.check(&TokenKind::Dedent) {
            return Ok(());
        }
        Err(ParserError {
            message: format!("Expected newline after statement, found '{}'", self.peek().kind),
            span: self.peek().span,
            code: "E0023",
        })
    }
}

