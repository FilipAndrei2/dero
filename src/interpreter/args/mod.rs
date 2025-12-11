pub mod args_parser_impl;

struct ArgsParsingResult;

trait ArgsParser {
    fn new() -> Box<dyn ArgsParser>;

    fn parse_args(&self) -> ArgsParsingResult;
}