use super::ArgsParser;
use super::ArgsParsingResult;
use std::env;

pub(crate) struct ArgsParserImpl {
    raw_args: env::Args
 }

impl ArgsParser for ArgsParserImpl {
    fn new() -> ArgsParser {
        let raw_args = std::env::args();    
        ArgsParserImpl {
            raw_args
        }
    }

    fn parse_args(&self) -> ArgsParsingResult {
        for arg in &(self.raw_args) {
            println!("{}", arg);
        }

        ArgsParsingResult { }
    }
}

impl ArgsParserImpl {
}
