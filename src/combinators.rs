use parser::Parser;

pub fn char(c: char) -> impl Parser<Output = char> {
    struct CharCombinator(char);

    impl Parser for CharCombinator {
        type Output = char;

        fn parse<'a>(&self, input: &'a str) -> (Option<Self::Output>, &'a str) {
            let mut input_chars = input.chars();

            if input_chars.next() == Some(self.0) {
                (Some(self.0), input_chars.as_str())
            } else {
                (None, input)
            }
        }
    }

    CharCombinator(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_works() {
        let parser = char('(');

        assert_eq!(parser.parse("()"), (Some('('), ")"));
        assert_eq!(parser.parse(")("), (None, ")("));
        assert_eq!(parser.parse(""), (None, ""));
    }
}
