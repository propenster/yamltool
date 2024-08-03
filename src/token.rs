

#[derive(Debug,  Clone,  PartialEq)]
pub enum TokenKind{ 
    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    COLON,
    COMMA,
    PERIOD,
    LSQUARE,
    RSQUARE,
    
    BLITERAL,
    NLITERAL,
    SLITERAL,
    
    ILLEGAL,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token{
    pub kind: TokenKind,
    pub literal: String,
}

impl Token{
    pub fn new(kind: TokenKind, literal: String) -> Token{
        Self { kind, literal}
    }
}