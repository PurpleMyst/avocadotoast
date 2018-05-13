use parser::Parser;

pub fn char(c: char) -> impl Parser<Output = char> {
    struct CharCombinator(char);

    impl Parser for CharCombinator {
        type Output = char;

        fn parse<'a>(&mut self, input: &'a str) -> (Option<Self::Output>, &'a str) {
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

pub fn predicate<P: Fn(char) -> bool>(predicate: P) -> impl Parser<Output = char> {
    struct PredicateCombinator<P: Fn(char) -> bool>(P);

    impl<P: Fn(char) -> bool> Parser for PredicateCombinator<P> {
        type Output = char;

        fn parse<'a>(&mut self, input: &'a str) -> (Option<Self::Output>, &'a str) {
            let mut input_chars = input.chars();

            match input_chars.next() {
                Some(c) if self.0(c) => (Some(c), input_chars.as_str()),
                _ => (None, input),
            }
        }
    }

    PredicateCombinator(predicate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_works() {
        let mut parser = char('(');

        assert_eq!(parser.parse("()"), (Some('('), ")"));
        assert_eq!(parser.parse(")("), (None, ")("));
        assert_eq!(parser.parse(""), (None, ""));
    }

    #[test]
    fn predicate_works() {
        let mut parser = predicate(|c| c.is_digit(10));

        assert_eq!(parser.parse("123"), (Some('1'), "23"));
        assert_eq!(
            parser.parse("this is not a number, i promise"),
            (None, "this is not a number, i promise")
        );
        assert_eq!(parser.parse(""), (None, ""));
    }
}
