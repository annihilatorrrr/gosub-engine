use super::new_tokenizer::{Token, TokenKind};
use crate::{bytes::CharIterator, css3::new_tokenizer::Tokenizer};

struct Function {
    name: String,
    value: Vec<ComponentValue>,
}

enum SimpleBlockTokenKind {
    Curly,
    Bracket,
    Paren,
}

struct SimpleBlock {
    kind: SimpleBlockTokenKind,
    value: Vec<ComponentValue>,
}

struct AtRule {
    name: String,
    prelude: Vec<ComponentValue>,
    block: Option<SimpleBlock>,
}

struct QualifiedRule {
    prelude: Vec<ComponentValue>,
    block: SimpleBlock,
}

struct Declaration {
    name: String,
    value: Vec<ComponentValue>,
    important: bool,
}

enum ComponentValue {
    /// Any token expect for `<function-token>`, `<{-token>`, `<(-token>`, and `<[-token>` (which are consumed in other higher-level objects)
    ///
    /// Note: `<}-token>`, `<)-token>`, `<]-token>`, `<bad-string-token>`, and `<bad-url-token>` are always parse errors.
    Token(Token),
    Function(Function),
    SimpleBlock(SimpleBlock),
}

// Parser output: at-rules, qualified rules, and/or declarations
pub struct CSS3Parser<'stream> {
    tokenizer: Tokenizer<'stream>,
}

impl<'stream> CSS3Parser<'stream> {
    pub fn new(tokenizer: Tokenizer) -> CSS3Parser {
        CSS3Parser { tokenizer }
    }

    pub fn from_input_stream(ci: &mut CharIterator) -> CSS3Parser {
        CSS3Parser::new(Tokenizer::new(ci))
    }

    /// [5.3.1. Parse something according to a CSS grammar](https://www.w3.org/TR/css-syntax-3/#parse-grammar)
    fn parse() {
        todo!()
    }

    /// [5.3.2. Parse A Comma-Separated List According To A CSS Grammar](https://www.w3.org/TR/css-syntax-3/#parse-comma-list)
    fn parse_comma_separated_list() {
        todo!()
    }

    /// [5.4.9. Consume a function](https://www.w3.org/TR/css-syntax-3/#consume-function)
    fn consume_function(&mut self) {
        let _name = self.consume_token(TokenKind::Function).value();
        let mut value = Vec::new();

        loop {
            let token = self.tokenizer.consume();

            if token.kind() == TokenKind::LParen || token.is_eof() {
                break;
            }

            value.push(token);
        }
    }

    /// [5.4.7. Consume a component value](https://www.w3.org/TR/css-syntax-3/#consume-a-component-value)
    fn consume_component_value(&mut self) {}

    /// [5.4.8. Consume a simple block](https://www.w3.org/TR/css-syntax-3/#consume-a-simple-block)
    fn consume_simple_block(&mut self) {
        let start = self.current_token().kind();
        // consume block start  <{-token>, <[-token>, or <(-token>.
        self.consume_token(start);

        let mut value = Vec::new();

        loop {
            // eof: parser error
            if self.current_token().kind() == start || self.current_token().is_eof() {
                break;
            }

            // todo: handle "component value" creation from a simple block

            value.push(self.consume_token(TokenKind::Any))
        }

        let _kind = match start {
            TokenKind::LParen => SimpleBlockTokenKind::Paren,
            TokenKind::LCurly => SimpleBlockTokenKind::Curly,
            TokenKind::LBracket => SimpleBlockTokenKind::Bracket,
            _ => todo!(),
        };
    }

    fn current_token(&self) -> Token {
        self.tokenizer.lookahead(0)
    }

    fn next_token(&self) -> Token {
        self.tokenizer.lookahead(1)
    }

    fn consume_token(&mut self, kind: TokenKind) -> Token {
        let token = self.tokenizer.consume();

        if kind != TokenKind::Any {
            // safeguard, not to consume unexpected token
            assert_eq!(token.kind(), kind);
        }

        token
    }
}
