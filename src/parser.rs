pub struct MapCombinator<I, O, P: Sized + Parser<Output = I>, F: Fn(I) -> Option<O>>(P, F);

impl<I, O, P: Parser<Output = I>, F: Fn(I) -> Option<O>> Parser for MapCombinator<I, O, P, F> {
    type Output = O;

    fn parse<'a>(&mut self, input: &'a str) -> (Option<Self::Output>, &'a str) {
        let (out, rest) = self.0.parse(input);
        (out.and_then(&self.1), rest)
    }
}

pub trait Parser {
    type Output;

    fn parse<'a>(&mut self, input: &'a str) -> (Option<Self::Output>, &'a str);

    fn map<O, F: Fn(Self::Output) -> Option<O>>(
        self,
        f: F,
    ) -> MapCombinator<Self::Output, O, Self, F>
    where
        Self: Sized,
    {
        MapCombinator(self, f)
    }
}
