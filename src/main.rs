mod parsers;

use parsers::{Parser, ParserState, ParserArray};

fn main() {
    let state0 = ParserState {
        index:0,
        result: None,
        error: false,
        err_msg: None,
    };
    let state1 = ParserState {
        index:0,
        result: None,
        error: false,
        err_msg: None,
    };
    let state2 = ParserState {
        index: 0,
        result: None,
        error: false,
        err_msg: None
    };


    let mut parser0 = Parser {
        substr: String::from("Hello world!"), 
        state: state0,
    };
    let mut parser1 = Parser {
        substr: String::from(" Goodbye..."), 
        state: state1,
    };

    let arr = vec![parser0, parser1];
    let mut parser = ParserArray {
        parsers: arr,
        state: state2,
    };

    parser.run(String::from("Hello world! Goodbye...It has been a cruel experience"));
}
