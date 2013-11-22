use token::{ Token };
use std::fmt;

pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>
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
		write!(f.buf, "Node : T({}), L[{}], R({})", obj.token, left, right)
	}
}

pub fn parse_expression(mut tokens: ~[Token], mut lhs: Node, min_precedence: uint) -> (Node, ~[Token]) {

	// lhs: left-hand-side, rhs: right-hand-side

	if tokens[0] == lhs.token { // lhs is not lookahead(lh)
		tokens.shift();
	}
	
	let mut lh = tokens[0]; // look at the next token, our lookahead

	while lh.is_operator() && lh.precedence() >= min_precedence {
		
		let op = tokens.shift(); // get next token, our operator
		let mut rhs = Node{ token: tokens.shift(), left: None, right: None }; // get next token, our rhs
		lh = tokens[0];

		while lh.is_operator() && lh.precedence() > op.precedence() {
			
			// recursive invocation of parse_expression 
			rhs = match parse_expression(tokens, rhs, lh.precedence()) {
				(r, t) => { tokens = t; r }
			};
			lh = tokens[0];

		}

		// save our results in the lhs
		lhs = Node{ token: op, left: Some(~lhs), right: Some(~rhs) };
		lh = tokens[0];

	}

	return (lhs, tokens);

}

#[test]
fn test_parse_expression() {
	
	use lexer::Lexer;
	use token::Number;

	let lexer = Lexer::new(~"3+4*7*7+6+3");

	match parse_expression(lexer.tokens, Node{ token: Token(Number(3f64)), left: None, right: None }, 0) {
		(n, _) => debug!("{}", n)	
	}

}
