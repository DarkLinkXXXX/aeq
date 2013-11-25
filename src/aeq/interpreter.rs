use token:: { Add, Sub, Mul, Div, Number, Assign, Identifier };
use parser;

#[deriving(Clone)]
struct Variable {
	name: ~str,
	value: f64,
	index: uint
}

struct SymbolTable {
	variables: ~[Variable]
}

impl SymbolTable {
	
	pub fn get_var(&mut self, identifier: ~str) -> Option<Variable> {
		
		let mut i = 0;		

		while i < self.variables.len() {
			
			match self.variables[i].name.clone() {
				
				var => {
					if var == identifier {
						self.variables[i].index = i;
						return Some(self.variables[i].clone())
					}
				}
			}

			i += 1;
		}

		return None

	}

	pub fn set_var(&mut self, identifier: ~str, value: f64) {
		
		match self.get_var(identifier.clone()) {
			Some(x) => self.variables[x.index].value = value,
			None    => self.variables.push(Variable{ name: identifier, value: value, index: 0})
		}

	}

}

// Implement a interpret method to interpret our parse tree.
impl parser::Node {

	pub fn interpret(&self, symboltable: &mut SymbolTable) -> f64{
		
		let left = match self.left {
			Some(ref n) => {
					match *self.token {
						Assign => {
							match *n.token {
								Identifier(_) => 1f64,
								_ => n.interpret(symboltable) // recursive invocation.
							}
						}
						_ => n.interpret(symboltable) // recursive invocation.
					}
				}
			None        => 1f64
		};

		let right = match self.right {
			Some(ref n) => n.interpret(symboltable), // recursive invocation.
			None        => 1f64
		};

		match *self.token {

			Add => left + right,
			Sub => left - right,
			Mul => left * right,
			Div => left / right,
			Assign => {
				match self.left {
					Some(ref n) => match *n.token {
						Identifier(ref s) => { symboltable.set_var(s.clone(), right); return right }
						_ => { 
							error!("[error: interpreter.rs in Node::interpret] -> Expected identifier."); return 0f64
						}
					},
					None => {
						error!("[error: interpreter.rs in Node::interpret] -> Expected identifier."); return 0f64 
					}
				}
			}
			Identifier(ref s) => { match symboltable.get_var(s.clone()) {
					
					Some(n) => n.value,
					None    => { error!("[error: interpreter.rs in Node::interpret] -> Variable {} dont exists.", *s); 0f64 }
					
				}
			}
			Number(n) => n,	
			_   => { error!("[error: interpreter.rs in Node::interpret] -> Unknown Token to interpret."); return 0f64 }

		}

	}

}	
