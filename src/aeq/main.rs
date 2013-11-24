use std::io::stdio::stdin;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod interpreter;

fn read_until(c: u8, include: bool) -> ~str {
	let mut bytes = ~[];
	loop {
		let ch = stdin().read_byte().unwrap();
		if ch == -1 || ch == c as u8 {
			if include && ch == c as u8 {
				bytes.push(ch as u8);
			}
			break;
		}
		bytes.push(ch as u8);
	}
	std::str::from_utf8(bytes)
}

fn main() {

	let mut symboltable = interpreter::SymbolTable{ variables: ~[] };

	loop {
		let text = read_until('\n' as u8, false);
		let lexer = lexer::Lexer::new(text.clone());
		let parser = parser::Parser::new(lexer);
		println!("{} = {}", text, parser.root.interpret(&mut symboltable));
	}
}


