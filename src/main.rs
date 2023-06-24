use pulldown_cmark::{Options, Parser};
use std::fs::File;
use std::io::Read;
use typmd::Typ;

fn main() {
    let mut file = File::open("test/in.md").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let opt = Options::all();
    let parser = Parser::new_ext(&input, opt);

    let mut output = String::new();
    let mut tpy = Typ::new(parser, &mut output);
    tpy.run().unwrap();

    print!("{output}");
}
