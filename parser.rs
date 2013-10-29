pub mod parser {

    pub struct Token {
        id: Tokens
    }

    impl Token {
        pub fn print(&self) {
            match self.id {
                Constant(Number(n))     => println!("{}", n),
                Operator(Add)           => println("Add"),
                Operator(Sub)           => println("Sub"),
                Operator(Mul)           => println("Mul"),
                Operator(Div)           => println("Div"),
                Parenthese(Open)        => println("Parenthese Open"),
                Parenthese(Close)       => println("Parenthese Close"),
                _                       => println("Unknown")
            }
        }
    }

    pub enum Tokens {
        Constant(Constants),
        Operator(Operators),
        Parenthese(Parentheses),
        Unknown
    }

    pub enum Constants {
        Number(f64)
    }

    pub enum Operators {
        Add, Sub, Mul, Div
    }

    pub enum Parentheses {
        Open, Close
    }

    pub fn each(t: &[Token], op: &fn(t: &Token)) {
        let mut n = 0;
        while n < t.len() {
            op(&t[n]);
            n += 1;
        }
    }

    // TODO Make more readable

    pub fn tokenizer(expr: ~str) -> ~[Token] {
        use std::char::from_u32;
        use std::char::is_digit;
        use std::from_str::from_str;

        let mut tokens: ~[Token] = ~[];
        
        let mut n = 0; 
        while n < expr.len() {
            let c = match from_u32(expr[n] as u32) {
                Some(cc) => cc,
                None     => return ~[]
            };
            match c {
                '+' => tokens.push(Token{ id: Operator(Add) }),
                '-' => tokens.push(Token{ id: Operator(Sub) }),
                '*' => tokens.push(Token{ id: Operator(Mul) }),
                '/' => tokens.push(Token{ id: Operator(Div) }),
                '(' => tokens.push(Token{ id: Parenthese(Open) }),
                ')' => tokens.push(Token{ id: Parenthese(Close) }),
                d   if is_digit(d) => {
                    let begin = n;
                    while n < expr.len()  {
                        let cc = match from_u32(expr[n] as u32) {
                            Some(ccc) => ccc,
                            None     => return ~[]
                        };
                        if !is_digit(cc) && cc != '.' {
                            break;
                        } 
                        n += 1;
                    }
                    let number = match from_str::<f64>(expr.slice(begin, n)) {
                        Some(n) => n,
                        None    => 0f64
                    };
                    tokens.push(Token{ id: Constant(Number(number)) });
                    n -= 1;
                }
                  _ => tokens.push(Token{ id: Unknown })
            }
            n += 1;
        }
        tokens
    }
}
