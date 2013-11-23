use token:: { Add, Sub, Mul, Div, Number };
use parser;

impl parser::Node {
	pub fn interprete(&self) -> f64{

		let left = match self.left {
			Some(ref n) => n.interprete(),
			None        => 1f64
		};

		let right = match self.right {
			Some(ref n) => n.interprete(),
			None        => 1f64
		};

		match *self.token {
			Add => left + right,
			Sub => left - right,
			Mul => left * right,
			Div => left / right,
			Number(n) => n,
			_   => { error!("Unknown Token to interprete."); return 0f64 }
		}
	}
}	

#[test]
fn test_interpreter() {
	use parser::parse_expression;
	use lexer::Lexer;
	use token::Token;

	let lexer = Lexer::new(~"22/7");
	let tokens = lexer.tokens.clone();
	match parse_expression(lexer.tokens, parser::Node{ token: tokens[0], left: None, right: None }, 0) {
		(n, _) => println!("{}", n.interprete())
	}
}
