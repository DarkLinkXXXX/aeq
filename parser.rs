use token::{ Token, tokenizer, Add, Sub, Mul, Div, Unknown, Number, OpenParentheses, CloseParentheses };

pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>,
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
	
	debug!("init: lhs = {}, minp = {}", fmt_token(lhs.token), min_precedence);
	let mut i = lhs.index;
	debug!("init: i = {}", i);
	let mut rhs: Node = Node{ token: Token(Unknown(' ')), left: None, right: None, index: 0 };

	while is_operator(tokens[i+1]) && precedence(tokens[i+1]) >= min_precedence {

		debug!("\twhile 1: {} and {} >= {}", is_operator(tokens[i+1]), precedence(tokens[i+1]), min_precedence);
		i += 1;
		debug!("\twhile 1: i = {}", i);
		let op = tokens[i];
		debug!("\twhile 1: op = {}", fmt_token(op));
		i += 1;
		debug!("\twhile 1: i = {}", i);
		rhs = Node{ token: tokens[i], left: Some(~rhs), right: None, index: i };
		debug!("\twhile 1: rhs = {}", fmt_token(rhs.token));

		debug!("\twhile 1: i = {}, len = {}", i, tokens.len());
		if i + 1 >= tokens.len() { break }

		while is_operator(tokens[i+1]) && precedence(tokens[i+1]) > precedence(op) {
			
			debug!("\t\twhile 2: {} and {} > {}", is_operator(tokens[i+1]), precedence(tokens[i+1]), precedence(op));
			let lookahead = tokens[i+1];
			debug!("\t\t\twhile 2: lookahead = {}", fmt_token(lookahead));
			rhs = parse_expression(tokens, rhs, precedence(lookahead));
			debug!("\t\t\twhile 2: rhs = {}", fmt_token(rhs.token));
			i = rhs.index + 1;
			debug!("\t\t\twhile 2: i = {}", i);
		}

		debug!("\twhile 1: lhs = {}, rhs = {}", fmt_token(lhs.token), fmt_token(rhs.token));

	}

	debug!("return ( {}, {}, {}, {} )", fmt_token(tokens[i-1]), fmt_token(lhs.token), fmt_token(rhs.token), i-1);
	return Node{ token: tokens[i-1], left: Some(~lhs), right: Some(~rhs), index: i-1 };

}

#[test]
fn test_parse_expression() {
	
	let expr = "3+4*7+6"; 
	debug!("{}", expr);
	parse_expression(tokenizer(expr), Node{ token: Token(Number(3f64)), left: None, right: None, index: 0}, 0);

}
