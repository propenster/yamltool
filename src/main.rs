use std::path::PathBuf;

use yamltoolrs::{lexer::Lexer, parser::{CsParser, Parser}, writer::OutputWriter};







///this was written in C# before,
/// I just wanted to experiment and see performance disparities
fn main() {
    let source = std::fs::read_to_string("./src/examples/csharp/testconfig.json").expect("Invalid file path");
    let lexer = Lexer::new(source);
    let mut parser = CsParser::new(lexer);

    let parsed_json = parser.parse();
    let mut writer = OutputWriter::new(parsed_json);
    let out_path = PathBuf::from("./src/output.yml");
    writer.write_output_yaml(out_path.as_path());


    println!();
    println!();
    println!("Thank you for using yamltoolrs 1.0");




}



#[cfg(test)]
mod tests{
    use yamltoolrs::{lexer::Lexer, token::TokenKind};



    #[test]
    fn test1(){
        let val = 29;
        assert_eq!(29, val);
    }

    #[test]
    fn test_scanner_emits_correct_tokens(){
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
    fn test_scanner_emits_correct_tokens_with_iterator(){
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
    fn test_scanner_skip_comments(){
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