use std::io::*;

pub mod parser;

fn main() {
    use parser::*;
    loop {
        do parser::each(parser::tokenizer(stdin().read_line())) |t| {
            t.print();
        }
    }
}
