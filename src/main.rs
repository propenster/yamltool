use std::{
    env::args,
    path::{Path, PathBuf},
};

use yamltoolrs::{
    lexer::Lexer,
    parser::{CsParser, Parser},
    writer::OutputWriter,
};

const USAGE: &str = "Usage:
yamltool <filePath> <language>

Arguments:
<filePath>  Path to the JSON or properties file this will be appsettings.json for .NET(C#) or application.properties for JAVA
<language>  Programming language option (--java or --csharp)
";

const VERSION: &str = "1.0.0";

///this was written in C# before,
/// I just wanted to experiment and see performance disparities
fn main() {
    let num_args = std::env::args().skip(1).len();
    let output: String = match num_args {
        0 => String::from(USAGE),
        1 => {
            let first = args().nth(1).unwrap();
            if first == "--help" || first == "-h" {
                String::from(USAGE)
            } else if first == "--version" || first == "-v" {
                format!("Version: {}", VERSION)
            } else {
                String::from(VERSION)
            }
        }
        2 => {
            let lang = args().nth(2).unwrap();
            if lang == "--java" {
                String::from(USAGE)
            } else if lang == "--csharp" {
                process_file(args().nth(1).unwrap().as_str(), lang)
            } else {
                String::from("Invalid language option. Use '--java' or '--csharp'.")
            }
        }

        _ => String::from("Invalid number of arguments. Use '--help' for usage instructions."),
    };

    println!("{}", output);

    println!();
    println!("Thank you for using yamltool 1.0");
}

fn process_file(path: &str, lang: String) -> String {
    if !Path::new(path).exists() {
        return format!("File '{}' does not exist", path);
    }
    if let Ok(source) = std::fs::read_to_string(path) {
        let mut lexer = Lexer::new(source);

        match lang.as_str() {
            "--java" => {
                return process_java(path, &mut lexer)
            }
            "--csharp" => {
                return process_csharp(path, &mut lexer)
            }
            _ => return String::from("Invalid language")
        }
    }

    String::from("Could not read input source file")
}

fn process_java(path: &str, lexer: &mut Lexer) -> String {
    
    println!("Path: {} \nLexer: {:?}", path, lexer);
    return String::new()
}

fn process_csharp(path: &str, lexer: &mut Lexer) -> String {
    let mut parser = CsParser::new(lexer);

    let parsed_json = parser.parse();
    if parsed_json.is_empty() {
        return format!("Could not parse input config file: {path}"); 
    }
    let mut writer = OutputWriter::new(parsed_json);
    let out_path = PathBuf::from("./output.yml"); //this is hardcoded... should user be able to pass it to args?
    writer.write_output_yaml(out_path.as_path());

    return format!("Yaml output generated: {:?}", out_path);
}

#[cfg(test)]
mod tests {
    use yamltoolrs::{lexer::Lexer, token::TokenKind};
    #[test]
    fn test1() {
        let val = 29;
        assert_eq!(29, val);
    }

    #[test]
    fn test_scanner_emits_correct_tokens() {
        let source: String = String::from("{ } \"echo\" true");

        let mut scanner = Lexer::new(source);

        let mut result = scanner.lex();

        assert_eq!(TokenKind::LCURLY, result.kind);

        result = scanner.lex();
        assert_eq!(TokenKind::RCURLY, result.kind);

        result = scanner.lex();
        assert_eq!(TokenKind::SLITERAL, result.kind);
        result = scanner.lex();
        assert_eq!(TokenKind::BLITERAL, result.kind);
    }

    #[test]
    fn test_scanner_emits_correct_tokens_with_iterator() {
        let source: String = String::from("{ } \"echo\" true ,");

        let mut scanner = Lexer::new(source);
        //not gonna lie, this idiomatic rust stuff made things uglier but ok I guess
        assert_eq!(TokenKind::LCURLY, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::RCURLY, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::SLITERAL, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::BLITERAL, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::COMMA, scanner.next().unwrap().kind);
    }

    #[test]
    fn test_scanner_skip_comments() {
        let source: String = String::from("{ //\"echo\" this is a comment\r\n:589,[]");

        let mut scanner = Lexer::new(source);
        assert_eq!(TokenKind::LCURLY, scanner.next().unwrap().kind);

        assert!(scanner.next().is_none()); //comment should / MUST not yield a Token
        assert_eq!(TokenKind::COLON, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::NLITERAL, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::COMMA, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::LSQUARE, scanner.next().unwrap().kind);
        assert_eq!(TokenKind::RSQUARE, scanner.next().unwrap().kind);
    }
}
