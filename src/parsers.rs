#[derive(Debug)]
pub struct ParserState {
    pub index: usize,
    pub result: Option<String>,
    pub error: bool,
    pub err_msg: Option<String>
}


pub struct Parser {
    pub substr: String,
    pub state: ParserState,
}

pub struct ParserArray {
    pub parsers: Vec<Parser>,
    pub state: ParserState
}

impl Parser {
    pub fn new(substr: String) -> Self {
        Parser{
            substr: substr,
            state: ParserState {
                index: 0,
                result: None,
                error: false,
                err_msg: None
            }
        }
    }

    pub fn run(&mut self, corpus: String) {
        let new_state;
        if corpus.len() < self.substr.len() {
            new_state = ParserState {
                index: 0,
                result: None,
                error: true,
                err_msg: Some(String::from("Corpus is of smaller length than target string")),
            };
        } else {
            if corpus[..self.substr.len()] == self.substr {
                new_state = ParserState {
                    index: self.state.index + self.substr.len(),
                    result: Some(corpus[self.state.index..self.state.index + self.substr.len()].to_string()),
                    error: false,
                    err_msg: None,
                };
            } else {
                new_state = ParserState {
                    index: 0,
                    result: None,
                    error: true,
                    err_msg: Some(String::from("Target string not found in corpus")),
                };
            }
        }
        self.state = new_state;
        ()
    }
}

impl ParserArray {
    pub fn new(parsers: Vec<Parser>) -> Self {
        ParserArray {
            parsers: parsers,
            state: ParserState {
                index: 0,
                result: None,
                error: false,
                err_msg: None
            }
        }
    }

    pub fn run(&mut self, corpus: String) {
        for parser in &self.parsers {
            let new_state;
            let dif = corpus.len() - self.state.index;
            if dif < parser.substr.len() {
                new_state = ParserState {
                    index: 0,
                    result: None,
                    error: true,
                    err_msg: Some(String::from("Corpus is of smaller length than target string")),
                };
            } else {
                if corpus[self.state.index..self.state.index + parser.substr.len()] == parser.substr {
                    new_state = ParserState {
                        index: self.state.index + parser.substr.len(),
                        result: Some(corpus[self.state.index..self.state.index + parser.substr.len()].to_string()),
                        error: false,
                        err_msg: None,
                    };
                } else {
                    new_state = ParserState {
                        index: 0,
                        result: None,
                        error: true,
                        err_msg: Some(String::from("Target string not found in corpus")),
                    };
                }
            }
            self.state = new_state;
        }
        println!("{:?}", self.state)
    }
}