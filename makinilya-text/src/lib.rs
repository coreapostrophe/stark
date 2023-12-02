use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar/makinilya.pest"]
pub struct MakinilyaParser;

#[cfg(test)]
mod parser_tests {
    use pest::Parser;

    use crate::{MakinilyaParser, Rule};

    #[test]
    fn parses_string_interpolation() {
        let file = MakinilyaParser::parse(Rule::file, "{{ name }}");
        assert!(file.is_ok());
        let file = MakinilyaParser::parse(Rule::file, "{{ }}");
        assert!(file.is_err());
        let file = MakinilyaParser::parse(Rule::file, "{{ 32 }}");
        assert!(file.is_err());
        let file = MakinilyaParser::parse(Rule::file, "{{ name32 }}");
        assert!(file.is_ok());
        let file = MakinilyaParser::parse(Rule::file, "{{ name_32 }}");
        assert!(file.is_ok());
    }

    #[test]
    fn parses_content() {
        let file = MakinilyaParser::parse(Rule::file, "Hello. My name is {{ name }}.");
        assert!(file.is_ok());
    }
}
