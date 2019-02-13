use crate::token::Token;

pub enum NodeType {
    Variable,
    Function,
    Comment,
}

pub struct Node {
    pub node_type: NodeType,
    pub token: Token,
    pub children: Vec<Node>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    root: Node,
}

impl Parser {
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<(), String> {
        self.tokens = tokens;

        while self.current < self.tokens.len() {
            let token = &self.tokens[self.current];

            let node = match token {
                Token::New => Some(self.parse_declaration()?),
                _ => None,
            };

            if let Some(node) = node {
                self.root.children.push(node);
            }

            self.current += 1;
        }

        Ok(())
    }

    fn next(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current]
    }

    fn parse_declaration(&mut self) -> Result<Node, String> {
        let token = match self.next() {
            // TODO: deal with new const, new static, new stock etc
            Token::Symbol(v) => Token::Symbol(v.clone()),
            token @ _ => return Err(format!("expected symbol, found {:?}", token)),
        };

        let node = Node {
            node_type: NodeType::Variable,
            token: token,
            children: Vec::new(),
        };

        match self.next() {
            Token::Semicolon => Ok(node),
            Token::Equal => self.parse_expression(),
            token @ _ => Err(format!("expected expression, found {:?}", token)),
        }
    }

    fn parse_expression(&mut self) -> Result<Node, String> {
        Err(String::from("not implemented"))
    }
}
