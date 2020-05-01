#[derive(Clone)]
pub struct Parser<F> 
    where F: Fn(ParserState) -> ParserState {
    pub transformer: F,
    pub state: ParserState
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParserState {
    pub target: String,
    pub index: usize,
    pub result: Vec<String>, // the container of eventual results from the parsing, with Some(result) or None. Should be a vetor
    pub error: bool,            // whether we've encountered an error; index -> 0, Some(err_msg)
    pub err_msg: Option<String> // Eventual error message
}

#[allow(dead_code)]
impl<F> Parser<F> 
    where
        F: Fn(ParserState) -> ParserState {
    
    pub fn new(f: F) -> Self {
        // creating a new Parser just means deciding on which closure (f) it applies
        Parser {
            transformer: f,
            state: ParserState {
                target: String::from(""),
                index: 0,
                result: vec![],
                error: false,
                err_msg: None
            }
        }
    }

    pub fn map<G>(&mut self, g: G)
    where
        G: Fn(ParserState) -> ParserState 
    {
        // we update the state of the parser by applying the closure `g` to its state. 
        self.state = g((self.state).clone());
    }

    pub fn chain(&mut self) {

    }

    pub fn run(mut self, corpus: String) -> Self {
        // run() returns a new parser, with an updated state, by applying the transformation function to the parser's 
        // own state. 
        self.state.target = corpus;
        self.state = (self.transformer)(self.state);
        self
    }
}
