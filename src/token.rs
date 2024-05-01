

#[derive(Debug,  Clone,  PartialEq)]
pub enum TokenKind{
    
    
    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    COLON,
    COMMA,
    LSQUARE,
    RSQUARE,


    
    BLITERAL,
    NLITERAL,
    SLITERAL,
    
    ILLEGAL,
    EOF,
}

// #[derive(Debug, PartialEq)]
// pub enum Token{


//     LCURLY(TokenKind,String),
//     RCURLY(TokenKind,String),
//     LPAREN(TokenKind,String),
//     RPAREN(TokenKind,String),
//     COLON(TokenKind,String),
//     COMMA(TokenKind,String),
//     LSQUARE(TokenKind,String),
//     RSQUARE(TokenKind,String),
//     BOOLEANLITERAL(TokenKind,String),
//     NUMERICLITERAL(TokenKind,String),
//     STRINGLITERAL(TokenKind,String),


//     ILLEGAL(TokenKind,String),
//     EOF(TokenKind,String)

// }

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