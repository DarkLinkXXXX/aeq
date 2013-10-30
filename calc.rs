use std::io::*;

pub mod parser;

fn main() {
    use parser::*;
    loop {
        do parser::tokenizer::each(parser::tokenizer(stdin().read_line())) |t| {
            t.print();
        }
    }
}
