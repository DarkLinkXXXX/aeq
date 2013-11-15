use token::{ Token, Add, Sub, Mul, Div, Number };
use std::fmt;

pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>,
	index: uint
}

impl fmt::Default for Node {
	fn fmt(obj: &Node, f: &mut fmt::Formatter) {
		let left = match obj.left {
			Some(ref x) => format!("{}", **x),
			None => ~"None"
		};
		let right = match obj.right {
			Some(ref x) => format!("{}", **x),
			None => ~"None"
		};
		write!(f.buf, "Node : T({}), L({}), R({}), I({})", obj.token, left, right, obj.index)
	}
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

pub fn parse_expression(tokens: &[Token], mut lhs: Node, min_precedence: uint) -> Node {

	debug!("===========================================");

	let mut i = lhs.index;
	
	i += 1;
	debug!("i = {}", i);
	
	debug!("while is_operator({}) && precedence({}) >= {}", is_operator(tokens[i]), precedence(tokens[i]), min_precedence);
	while is_operator(tokens[i]) && precedence(tokens[i]) >= min_precedence {
		
		debug!("\toperator = {}", tokens[i]);
		let operator = tokens[i];

		i += 1;
		debug!("\ti = {}", i);

		let mut rhs = Node{ token: tokens[i], left: None, right: None, index: i }; 
		debug!("\t{}", rhs);

		debug!("\tif {} >= {}", i+1, tokens.len());
		if i+1 >= tokens.len() {
			lhs =  Node{ token: operator, left: Some(~lhs), right: Some(~rhs), index: i-1 };
			break
		}

		debug!("\twhile is_operator({}) && precedence({}) > {}", is_operator(tokens[i+1]), precedence(tokens[i+1]), precedence(operator));
		while is_operator(tokens[i+1]) && precedence(tokens[i+1]) > precedence(operator) {

			debug!("\t\tif {} >= {}", i+1, tokens.len());
			if i+1 >= tokens.len() {
				break
			}
			debug!("\t\t{}", rhs);
			rhs = parse_expression(tokens, rhs, precedence(tokens[i+1]));
			debug!("\t\t{}", rhs);
			i = match rhs.right {
				Some(ref x) => x.index + 1,
				None => fail!("FAIL; {}", rhs)
			};
			debug!("\t\ti = {}", i);
		}

		lhs =  Node{ token: operator, left: Some(~lhs), right: Some(~rhs), index: i-1 };
		debug!("\tlhs = {}", lhs);
	} 

	debug!("{}", lhs);
	return lhs;
}

#[test]
fn test_parse_expression() {
	
	use token::tokenizer;

	let expr = "3+4*7+6"; 
	parse_expression(tokenizer(expr), Node{ token: Token(Number(3f64)), left: None, right: None, index: 0}, 0);

}
