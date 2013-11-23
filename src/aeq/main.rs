use std::os::args;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod interpreter;

fn main() {

	if args().len() < 2 {
		println("Usage: ./aeq i\"(3+3*4)\"");
		return;
	}

	let lexer = lexer::Lexer::new(args()[1]);
	let parser = parser::Parser::new(lexer);
	println!("{} = {}", args()[1], parser.root.interprete());
}
