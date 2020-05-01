mod parser;
mod parsers;


#[allow(unused_imports)]
use parser::*;
use parsers::*;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string() {
        let haystack: String = String::from("Hello!Goodbye!");
        let needle = String::from("Hello!");
        let str_parser = Parser::new(str_parser(needle));
        let result = str_parser.run(haystack);
        let adv = ParserState {
            target: "Hello!Goodbye!".to_string(),
            index: 6,
            result: vec!["Hello!".to_string()],
            error: false,
            err_msg: None
        };
        assert_eq!(adv, result.state);
    }

    #[test]
    fn sequence() {
        let haystack: String = String::from("Hello!Goodbye!");
        let needle0 = String::from("Hello!");
        let needle1 = String::from("Goodbye!");
        let str_parser0 = Parser::new(str_parser(needle0));
        let str_parser1 = Parser::new(str_parser(needle1));
        let sequence = vec![str_parser0, str_parser1];
        let seq_par = Parser::new(seq_parser(&sequence));
        let result = seq_par.run(haystack);
        let adv = ParserState {
            target: "Hello!Goodbye!".to_string(),
            index: 14,
            result: vec!["Hello!".to_string(), "Goodbye!".to_string()],
            error: false,
            err_msg: None           
        }; 
        assert_eq!(adv, result.state);
    }

    #[test]
    fn map() {
        let haystack: String = String::from("Hello!Goodbye!");
        let needle = String::from("Hello!");
        let str_parser = Parser::new(str_parser(needle));
        let closure = |mut state: ParserState| {
            state.index = 0;
            state
        };
        let mut result = str_parser.run(haystack);
        result.map(closure);
        let adv = ParserState {
            target: "Hello!Goodbye!".to_string(),
            index: 0,
            result: vec!["Hello!".to_string()],
            error: false,
            err_msg: None
        };
        assert_eq!(adv, result.state);
    }
}