pub struct Parser<F> 
    where F: Fn(ParserState) -> ParserState {
    pub transformer: F
}

#[derive(Debug, PartialEq)]
pub struct ParserState {
    pub target: String,
    pub index: usize,
    pub result: Option<String>, // the container of eventual results from the parsing, with Some(result) or None
    pub error: bool,            // whether we've encountered an error; index -> 0, Some(err_msg)
    pub err_msg: Option<String> // Eventual error message
}

impl<F> Parser<F> 
    where F: Fn(ParserState) -> ParserState {
    pub fn new(f: F) -> Self {
        // creating a new Parser just means deciding on which closure it applies
        Parser {
            transformer: f
        }
    }

    pub fn run(&self, corpus: String) -> ParserState {
        let base_state = ParserState {
            target: corpus,
            index: 0,
            result: None,
            error: false,
            err_msg: None
        };
        return (self.transformer)(base_state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn str_parser(needle: String, state: ParserState) -> ParserState {
        let target_string = state.target;
        let index = state.index;
        if target_string[index..needle.len()] == needle {
            return ParserState {
                target: target_string,
                index: index + needle.len(),
                result: Some(needle),
                error: false,
                err_msg: None
            }
        } else {
            return ParserState {
                target: String::from(""),
                index: 0,
                result: None,
                error: true,
                err_msg: Some(String::from("Error"))
            }
        }
    }

    #[test]
    fn test_str_parser() {
        let mut haystack: String = String::from("Hello!Goodbye!");
        let mut strParser = Parser::new(move |x| str_parser(String::from("Hello"), x));
        let result = strParser.run(haystack);
        let adv = ParserState {
            target: "Hello!Goodbye!".to_string(),
            index: 5,
            result: Some("Hello".to_string()),
            error: false,
            err_msg: None
        };
        assert_eq!(adv, result);
    }
}