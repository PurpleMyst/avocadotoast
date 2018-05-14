use parser::Parser;

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

pub fn char(c: char) -> impl Parser<Output = char> {
    predicate(move |ic| c == ic)
}

pub fn many<O, P: Parser<Output = O>>(parser: P) -> impl Parser<Output = Vec<O>> {
    struct ManyCombinator<O, P: Parser<Output = O>>(P);

    impl<O, P: Parser<Output = O>> Parser for ManyCombinator<O, P> {
        type Output = Vec<O>;

        fn parse<'a>(&mut self, mut input: &'a str) -> (Option<Self::Output>, &'a str) {
            let mut result = Vec::new();

            while let (Some(out), new_input) = self.0.parse(input) {
                result.push(out);
                input = new_input;
            }

            (Some(result), input)
        }
    }

    ManyCombinator(parser)
}

pub fn many1<O, P: Parser<Output = O>>(parser: P) -> impl Parser<Output = Vec<O>> {
    many(parser).map(|out| if out.len() == 0 { None } else { Some(out) })
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn char_works() {
        let mut parser = char('(');

        assert_eq!(parser.parse("()"), (Some('('), ")"));
        assert_eq!(parser.parse(")("), (None, ")("));
        assert_eq!(parser.parse(""), (None, ""));
    }

    #[test]
    fn many_works() {
        let mut parser = many(char('('));

        assert_eq!(parser.parse("((()))"), (Some(vec!['(', '(', '(']), ")))"));
        assert_eq!(parser.parse(")))((("), (Some(vec![]), ")))((("));
    }

    #[test]
    fn many1_works() {
        let mut parser = many1(char('('));

        assert_eq!(parser.parse("((()))"), (Some(vec!['(', '(', '(']), ")))"));
        assert_eq!(parser.parse(")))((("), (None, ")))((("));
    }
}
