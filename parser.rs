use token::*;

pub struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>
}

pub fn parser(tokens: ~[Token]) -> Node {
	
	let stack: ~[Node]  = ~[];
	// Iterate through all tokens and generate a parse tree. 
	do each(tokens) |t| {
		
		let last_node = match stack.pop_opt() {
			Some(n) => n,
			None  	=> Node { token: Token(Unknown(' ')), left: None, right: None }
		};

		stack.push(Node{ token: *t, left: None, right: None });
	}

	return Node { token: Token(Unknown(' ')), left: None, right: None };
}

#[test]
fn test_parser() {
}
