pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    const PATH: &'static str = "src/contracts/helloworld.sol";
    use std::fs;

    use slang_solidity::parser::{ParseOutput, Parser};
    use slang_solidity::utils::LanguageFacts;

    #[test]
    fn test_hello_world() {
        let source = fs::read_to_string(PATH).unwrap();
        let parser = Parser::create(LanguageFacts::LATEST_VERSION).unwrap();
        let parse_output: ParseOutput = parser.parse_file_contents(&source);
        println!("{:?}", parse_output);
    }
}
