pub trait Parser {
    type Output;

    fn parse<'a>(&mut self, input: &'a str) -> (Option<Self::Output>, &'a str);
}

// TODO: Fix this.
//impl<'a, O, F: Fn(&'a str) -> (O, &'a str)> Parser for F {
//    type Output = O;
//
//    fn parse(&self, input: &'a str) -> (Self::Output, &'a str) {
//        self(input)
//    }
//}
