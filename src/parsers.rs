use crate::parser::*;
/*
    TODO: str_parser and seq_parser return parser states. 
    Edit so that they only modify a parser's inner state
*/
#[allow(dead_code)]
pub fn str_parser(needle: String) -> impl Fn(ParserState) -> ParserState {
    move |mut state: ParserState| {
        let target_string = state.target;
        let index = state.index;
        if target_string[index..index + needle.len()] == needle {
            state = ParserState {
                target: target_string,
                index: index + needle.len(),
                result: vec![needle.clone()],
                error: false,
                err_msg: None
            }
        } else {
            state = ParserState {
                target: String::from(""),
                index: 0,
                result: vec![],
                error: true,
                err_msg: Some(String::from("Error"))
            }
        }
        state
    }
}

#[allow(dead_code)]
pub fn seq_parser<F>(parsers: &[Parser<F>]) -> impl Fn(ParserState) -> ParserState + '_
    where F: Fn(ParserState) -> ParserState {
    move |state: ParserState| {
        let mut results: Vec<String> = Vec::with_capacity(parsers.len());
        let mut next_state = state;
        for parser in parsers {
            next_state = (parser.transformer)(next_state);
            results.push(next_state.clone().result[0].clone());
        }
        next_state.result = results;
        next_state
    }
}

// pub fn letters_parser() -> impl Fn(ParserState) -> ParserState {
//     unimplemented!()
// }

// pub fn digits_parser() -> impl Fn(ParserState) -> ParserState {
//     unimplemented!()
// }