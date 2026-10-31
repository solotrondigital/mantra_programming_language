// ==============================================================================
// mantra_lsp::semantic_tokens — Semantic Token Provider
// Walks the AST (or falls back to lexer tokens) to emit semantic highlights.
// ==============================================================================

use crate::analysis::AnalysisResult;
use crate::document::Document;
use crate::protocol::{semantic_token_modifier, semantic_token_type};
use mantrac_ast::{ClassMember, Declaration};
use mantrac_lexer::{token::TokenKind, Lexer};

/// A single semantic token (before delta encoding).
struct RawToken {
    line: u32,
    start: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
}

/// Compute semantic tokens for the whole document.
/// Returns the LSP-encoded flat integer array as a JSON array string.
pub fn get_semantic_tokens_json(document: &Document, analysis: &AnalysisResult) -> String {
    let mut raw: Vec<RawToken> = Vec::new();

    // ── AST-based tokens (authoritative) ─────────────────────────────────
    if let Some(program) = &analysis.program {
        for decl in &program.declarations {
            ast_tokens_decl(decl, &mut raw);
        }
    }

    // ── Lexer-based fallback for literals & operators ─────────────────────
    lexer_tokens(document, &mut raw);

    // ── Sort by position ──────────────────────────────────────────────────
    raw.sort_by(|a, b| a.line.cmp(&b.line).then(a.start.cmp(&b.start)));
    raw.dedup_by(|a, b| a.line == b.line && a.start == b.start);

    // ── Delta encode ──────────────────────────────────────────────────────
    let mut data: Vec<u32> = Vec::with_capacity(raw.len() * 5);
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;
    for tok in &raw {
        let delta_line = tok.line - prev_line;
        let delta_start = if delta_line == 0 {
            tok.start - prev_start
        } else {
            tok.start
        };
        data.push(delta_line);
        data.push(delta_start);
        data.push(tok.length);
        data.push(tok.token_type);
        data.push(tok.token_modifiers);
        prev_line = tok.line;
        prev_start = tok.start;
    }

    let nums: Vec<String> = data.iter().map(|n| n.to_string()).collect();
    format!(r#"{{"data":[{}]}}"#, nums.join(","))
}

// ─── AST walkers ─────────────────────────────────────────────────────────────

fn ast_tokens_decl(decl: &Declaration, out: &mut Vec<RawToken>) {
    match decl {
        Declaration::Class(cls) => {
            // Class name
            push_span(
                cls.span,
                0,
                cls.name.len() as u32,
                semantic_token_type::CLASS,
                semantic_token_modifier::DECLARATION | semantic_token_modifier::DEFINITION,
                out,
            );
            for member in &cls.members {
                match member {
                    ClassMember::Method(m) | ClassMember::StaticMethod(m) => {
                        let mods = if m.name == "__init__" {
                            semantic_token_modifier::DECLARATION
                        } else {
                            let mut mods = semantic_token_modifier::DECLARATION;
                            if m.is_static { mods |= semantic_token_modifier::STATIC; }
                            if m.is_abstract { mods |= semantic_token_modifier::ABSTRACT; }
                            if m.is_async { mods |= semantic_token_modifier::ASYNC; }
                            mods
                        };
                        let tok_type = if m.name == "__init__" {
                            semantic_token_type::CONSTRUCTOR
                        } else {
                            semantic_token_type::METHOD
                        };
                        push_span(m.span, 0, m.name.len() as u32, tok_type, mods, out);
                        for param in &m.params {
                            push_span(
                                param.span,
                                0,
                                param.name.len() as u32,
                                semantic_token_type::PARAMETER,
                                0,
                                out,
                            );
                        }
                    }
                    ClassMember::Field(f) => {
                        push_span(
                            f.span,
                            0,
                            f.name.len() as u32,
                            semantic_token_type::FIELD,
                            semantic_token_modifier::DECLARATION,
                            out,
                        );
                    }
                    ClassMember::StaticField(f) => {
                        push_span(
                            f.span,
                            0,
                            f.name.len() as u32,
                            semantic_token_type::FIELD,
                            semantic_token_modifier::DECLARATION | semantic_token_modifier::STATIC,
                            out,
                        );
                    }
                    ClassMember::Property(p) => {
                        push_span(
                            p.span,
                            0,
                            p.name.len() as u32,
                            semantic_token_type::PROPERTY,
                            semantic_token_modifier::DECLARATION,
                            out,
                        );
                    }
                }
            }
        }
        Declaration::Interface(iface) => {
            push_span(
                iface.span,
                0,
                iface.name.len() as u32,
                semantic_token_type::INTERFACE,
                semantic_token_modifier::DECLARATION | semantic_token_modifier::DEFINITION,
                out,
            );
            for method in &iface.methods {
                push_span(
                    method.span,
                    0,
                    method.name.len() as u32,
                    semantic_token_type::METHOD,
                    semantic_token_modifier::ABSTRACT,
                    out,
                );
            }
        }
        Declaration::Function(f) => {
            let mods = {
                let mut m = semantic_token_modifier::DECLARATION | semantic_token_modifier::DEFINITION;
                if f.is_async { m |= semantic_token_modifier::ASYNC; }
                m
            };
            push_span(f.span, 0, f.name.len() as u32, semantic_token_type::FUNCTION, mods, out);
            for param in &f.params {
                push_span(
                    param.span,
                    0,
                    param.name.len() as u32,
                    semantic_token_type::PARAMETER,
                    0,
                    out,
                );
            }
        }
        Declaration::Module(name, span) => {
            push_span(*span, 0, name.len() as u32, semantic_token_type::NAMESPACE, 0, out);
        }
        _ => {}
    }
}

fn push_span(
    span: mantrac_lexer::span::Span,
    _col_offset: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
    out: &mut Vec<RawToken>,
) {
    if span.line == 0 || length == 0 {
        return;
    }
    let line = (span.line - 1) as u32;
    let start = if span.col > 0 { (span.col - 1) as u32 } else { 0 };
    out.push(RawToken { line, start, length, token_type, token_modifiers });
}

// ─── Lexer-based token overlay ────────────────────────────────────────────────

fn lexer_tokens(document: &Document, out: &mut Vec<RawToken>) {
    let mut lexer = Lexer::new(&document.text);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(_) => return,
    };

    for tok in &tokens {
        let span = tok.span;
        if span.line == 0 {
            continue;
        }
        let line = (span.line - 1) as u32;
        let start = if span.col > 0 { (span.col - 1) as u32 } else { 0 };
        let (tok_type, mods, len) = match &tok.kind {
            TokenKind::Integer(_) => {
                let len = (span.end - span.start) as u32;
                (semantic_token_type::NUMBER, 0u32, len)
            }
            TokenKind::Float(_) => {
                let len = (span.end - span.start) as u32;
                (semantic_token_type::NUMBER, 0u32, len)
            }
            TokenKind::StringLit(s) => {
                let len = (s.len() + 2) as u32; // include quotes
                (semantic_token_type::STRING, 0u32, len)
            }
            TokenKind::Module => (semantic_token_type::KEYWORD, 0, 6),
            TokenKind::Import => (semantic_token_type::KEYWORD, 0, 6),
            TokenKind::From => (semantic_token_type::KEYWORD, 0, 4),
            TokenKind::As => (semantic_token_type::KEYWORD, 0, 2),
            TokenKind::Def => (semantic_token_type::KEYWORD, 0, 3),
            TokenKind::Return => (semantic_token_type::KEYWORD, 0, 6),
            TokenKind::Class => (semantic_token_type::KEYWORD, 0, 5),
            TokenKind::Interface => (semantic_token_type::KEYWORD, 0, 9),
            TokenKind::Extends => (semantic_token_type::KEYWORD, 0, 7),
            TokenKind::Implements => (semantic_token_type::KEYWORD, 0, 10),
            TokenKind::Abstract => (semantic_token_type::MODIFIER, 0, 8),
            TokenKind::Override => (semantic_token_type::MODIFIER, 0, 8),
            TokenKind::Static => (semantic_token_type::MODIFIER, 0, 6),
            TokenKind::Pub => (semantic_token_type::MODIFIER, 0, 3),
            TokenKind::Private => (semantic_token_type::MODIFIER, 0, 7),
            TokenKind::Protected => (semantic_token_type::MODIFIER, 0, 9),
            TokenKind::Let => (semantic_token_type::KEYWORD, 0, 3),
            TokenKind::Mut => (semantic_token_type::KEYWORD, 0, 3),
            TokenKind::If => (semantic_token_type::KEYWORD, 0, 2),
            TokenKind::Elif => (semantic_token_type::KEYWORD, 0, 4),
            TokenKind::Else => (semantic_token_type::KEYWORD, 0, 4),
            TokenKind::For => (semantic_token_type::KEYWORD, 0, 3),
            TokenKind::In => (semantic_token_type::KEYWORD, 0, 2),
            TokenKind::While => (semantic_token_type::KEYWORD, 0, 5),
            TokenKind::Break => (semantic_token_type::KEYWORD, 0, 5),
            TokenKind::Continue => (semantic_token_type::KEYWORD, 0, 8),
            TokenKind::Async => (semantic_token_type::MODIFIER, semantic_token_modifier::ASYNC, 5),
            TokenKind::Await => (semantic_token_type::KEYWORD, 0, 5),
            TokenKind::True | TokenKind::False | TokenKind::None => {
                let len = match &tok.kind {
                    TokenKind::True => 4,
                    TokenKind::False => 5,
                    TokenKind::None => 4,
                    _ => 0,
                };
                (semantic_token_type::KEYWORD, 0, len)
            }
            TokenKind::SelfKw => (semantic_token_type::KEYWORD, 0, 4),
            TokenKind::Super => (semantic_token_type::KEYWORD, 0, 5),
            TokenKind::And | TokenKind::Or | TokenKind::Not => {
                let len = match &tok.kind {
                    TokenKind::And => 3,
                    TokenKind::Or => 2,
                    TokenKind::Not => 3,
                    _ => 0,
                };
                (semantic_token_type::KEYWORD, 0, len)
            }
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
            | TokenKind::Percent | TokenKind::Eq | TokenKind::EqEq | TokenKind::NotEq
            | TokenKind::Lt | TokenKind::Gt | TokenKind::LtEq | TokenKind::GtEq
            | TokenKind::PlusEq | TokenKind::MinusEq | TokenKind::StarEq | TokenKind::SlashEq
            | TokenKind::Arrow => {
                let len = (span.end - span.start).max(1) as u32;
                (semantic_token_type::OPERATOR, 0u32, len)
            }
            TokenKind::At => (semantic_token_type::DECORATOR, 0, 1),
            _ => continue,
        };
        if len > 0 {
            out.push(RawToken { line, start, length: len, token_type: tok_type, token_modifiers: mods });
        }
    }
}

