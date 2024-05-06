use std::{
    collections::HashMap,
    fmt::{self},
};

use crate::{
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone )]
pub struct Float(f32);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < f32::EPSILON
    }
}
impl Eq for Float {}

///This is Object... holds reference 
/// to all object types in our PARSE-tree
#[derive(Eq, Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Boolean(bool),
    Number(Float),
    Dictionary(HashMap<String, Object>),
    List(Vec<HashMap<String, String>>),
}

impl Object {
    pub fn to_string(&self) -> String {
        match self {
            Object::String(s) => s.clone(),
            Object::Boolean(b) => b.to_string(),
            Object::Number(f) => f.0.to_string(),
            Object::Dictionary(_) => String::from("Dictionary"),
            Object::List(_) => String::from("List"),
        }
    }
    pub fn is_empty(&self) -> bool{
        match self{
            Object::Dictionary(obj) => obj.is_empty(),
            Object::String(s) => s.is_empty(),
            _ => false
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n.0),
            Object::Dictionary(dict) => {
                write!(f, "{{")?; //open dict
                let mut comma = false;
                for (key, value) in dict {
                    if comma {
                        write!(f, ", ")?;
                    } else {
                        comma = true;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}") //close dict
            }
            Object::List(list) => {
                write!(f, "[")?; //open list
                let mut comma = false;
                for item in list {
                    if comma {
                        write!(f, ", ")?;
                    } else {
                        comma = true;
                    }
                    write!(f, "{:?}", item)?;
                }
                write!(f, "]") //end of list
            }
        }
    }
}

impl Into<HashMap<String, Object>> for Object{
    fn into(self) -> HashMap<String, Object> {
        match self{
            Object::Dictionary(dict) => dict,
            _ => todo!()
        }
    }
}
///General trait for PARSING...
/// All variants of our parser (e.g CSharpParser, JAVAParser etc) will implement methods in this trait
pub trait Parser {
    fn parse(&mut self) -> Object;
    fn next(&mut self);
}

///C# (C-Sharp) Parser - we are not parsing C# as a language, we are passing C#-based config files e.g appsettings.json
///most C# projects have either appsettings.json [.JSON] or App.config...[XML]
pub struct CsParser<'a> {
    pub lexer: &'a mut Lexer,
    pub current_token: Token,
    pub next_token: Token,
}
///Csharp parser-specific methods
/// to be honest, I feel like this should be called JSONConfigParser but whatever
/// let's just go with this for now..
impl<'a> CsParser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = CsParser {
            lexer,
            current_token: Token::new(TokenKind::EOF, "\0".to_string()),
            next_token: Token::new(TokenKind::EOF, "\0".to_string()),
        };
        parser.next();
        parser
    }

    fn make_dictionary(&mut self) -> Object {
        let mut out_dict: HashMap<String, Object> = HashMap::new();
        while self.current_token.kind != TokenKind::RCURLY {
            let key = self.parse();
            if self.next_token.kind != TokenKind::COLON {
                panic!(
                    "Expected a colon. Found {}",
                    self.next_token.literal.to_string()
                );
            }
            self.next();
            let value = self.parse();

            out_dict.insert(key.to_string(), value);
            self.next();
        }

        return Object::Dictionary(out_dict);
    }

    fn make_list(&self) -> Object {
        todo!()
    }
}
///parser for C#-based config files e.g appsettings.json
impl<'a> Parser for CsParser<'a> {
    fn parse(&mut self) -> Object {
        self.next();
        if self.current_token.kind == TokenKind::SLITERAL {
            return Object::String(self.current_token.literal.to_string());
        } else if self.current_token.kind == TokenKind::NLITERAL {
            return Object::Number(Float(self.current_token.literal.parse::<f32>().unwrap()));
        } else if self.current_token.kind == TokenKind::BLITERAL {
            return Object::Boolean(str_to_bool(self.current_token.literal.as_str()).unwrap());
        } else if self.current_token.kind == TokenKind::LCURLY {
            return self.make_dictionary();
        } else if self.current_token.kind == TokenKind::LSQUARE {
            return self.make_list();
        }
        Object::String(String::new())
    }

    fn next(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next().unwrap_or(Token::new(TokenKind::EOF, String::new()));
    }
}

fn str_to_bool(input: &str) -> Result<bool, String> {
    match input.trim().to_lowercase().as_str() {
        "true" | "1" => Ok(true),
        "false" | "0" => Ok(false),
        _ => Err(format!("Invalid boolean value: '{}'", input)),
    }
}
