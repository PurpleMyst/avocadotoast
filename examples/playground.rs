// this is not actually an example, rather me testing stuff out.
extern crate avocadotoast;

use avocadotoast::Parser;

fn main() {
    let mut parser = avocadotoast::char('(');

    println!("{:?}", parser.parse("(x)"));
    println!("{:?}", parser.parse(")x("));
}
