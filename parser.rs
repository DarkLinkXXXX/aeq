use token::*;

pub struct Node {
	token: Token,
	left: Option<Node>,
	right: Option<Node>,
	index: uint
}

fn precedence(token: Token) -> uint {
	match *token {
		Add => 1, Sub => 1,
		Mul => 2, Div => 1,
		_ => 0
	}
}

fn is_operator(token: Token) -> bool {
	match *token {
		Add => true, Sub => true,
		Mul => true, Div => true,
		_ => false
	}
}

fn fmt_token(token: Token) -> ~str {
	match *token {
		Add => ~"Add",
		Sub => ~"Sub",
		Mul => ~"Mul",
		Div => ~"Div",
		Number(x) => format!("{}", x),
		OpenParentheses => ~"(",
		CloseParentheses => ~")",
		Unknown(x) => format!("{}", x)
	}
}

pub fn parse_expression(tokens: &[Token], lhs: Node, min_precedence: uint) -> Node {
	
	debug!(format!("init: lhs = {}, minp = {}", fmt_token(lhs.token), min_precedence));
	let mut i = lhs.index;
	debug!(format!("init: i = {}", i));
	let mut rhs: Node = Node{ token: Token(Add), left: None, right: None, index: 0 };

	while is_operator(tokens[i+1]) && precedence(tokens[i+1]) >= min_precedence {

		debug!(format!("\twhile 1: {} and {} >= {}", is_operator(tokens[i+1]), precedence(tokens[i+1]), min_precedence));
		i += 1;
		debug!(format!("\twhile 1: i = {}", i));
		let op = tokens[i];
		debug!(format!("\twhile 1: op = {}", fmt_token(op)));
		i += 1;
		debug!(format!("\twhile 1: i = {}", i));
		rhs = Node{ token: tokens[i], left: None, right: None, index: i };
		debug!(format!("\twhile 1: rhs = {}", fmt_token(rhs.token)));

		debug!(format!("\twhile 1: i = {}, len = {}", i, tokens.len()));
		if i + 1 >= tokens.len() { break }

		while is_operator(tokens[i+1]) && precedence(tokens[i+1]) > precedence(op) {
			
			debug!(format!("\t\twhile 2: {} and {} > {}", is_operator(tokens[i+1]), precedence(tokens[i+1]), precedence(op)));
			let lookahead = tokens[i+1];
			debug!(format!("\t\twhile 2: lookahead = {}", fmt_token(lookahead)));
			rhs = parse_expression(tokens, rhs, precedence(lookahead));
			debug!(format!("\t\twhile 2: rhs = {}", fmt_token(rhs.token)));
			i = rhs.index + 1;
			debug!(format!("\t\twhile 2: i = {}", i));
		}

		debug!(format!("\t while 1: lhs = {}, rhs = {}", fmt_token(lhs.token), fmt_token(rhs.token)));

	}

	debug!(format!("return ( {}, {}, {}, {} )", fmt_token(tokens[i-1]), fmt_token(lhs.token), fmt_token(rhs.token), i-1));
	return Node{ token: tokens[i-1], left: Some(lhs), right: Some(rhs), index: i-1 };

}

#[test]
fn test_parse_expression() {
	
	parse_expression(tokenizer("3+4*7+6"), Node{ token: Token(Number(7f64)), left: None, right: None, index: 0}, 0);

}
