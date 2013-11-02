use token::{ Token };

struct Node {
	token: Token,
	left: Option<~Node>,
	right: Option<~Node>
}

enum Expressions {
}

struct Expression(~[Token]);
