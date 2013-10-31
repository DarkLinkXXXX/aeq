pub mod token {

    use std::char::is_digit;
    use std::from_str::from_str;

    pub struct Token {
        id: Tokens
    }

    pub enum Tokens {
        Constant(Constants),
        Operator(Operators),
        Parenthesis(Parentheses),
        Unknown(char)
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

    pub fn tokenizer(text: &str) -> ~[Token] {

        let mut tokens: ~[Token] = ~[];
        
        //  Iterate through every character in the text.
        //  Check if the character is a token or could indicate some token.
        do iter(text) |ch, next| {

            let token = match ch {
                '+' => { Token{ id: Operator(Add) } }
                '-' => { Token{ id: Operator(Sub) } }
                '*' => { Token{ id: Operator(Mul) } }
                '/' => { Token{ id: Operator(Div) } }
                '(' => { Token{ id: Parenthesis(Open) } }
                ')' => { Token{ id: Parenthesis(Close) } }

                d if is_digit(d) => {
                    match token_number(d, next, text) {
                        Some(t) => { t }
                        None    => {
                            warn!("warning: token.rs in tokenizer: couldn't match token_number!");
                            Token{ id: Unknown(ch) } 
                        }
                    }
                }

                _   => {
                    info!(format!("info: token.rs in tokenizer: {} is a unknown character.", ch));
                    Token{ id: Unknown(ch) } 
                }
            }; 

            tokens.push(token);
        }

        return tokens;
    }

    fn token_number(ch: char, next: &mut uint, text: &str) -> Option<Token> {

        let mut number = ~"";

        // push the first given character ch e.g. 7.88 -> '7' would be ch
        // into our number string
        number.push_char(ch);

        // Iterate through the text until we hit the end of the number.
        // So we pushed every character of the number into the number string.
        loop {
            if *next >= text.len() {
                break
            }

            let ch = text.char_range_at(*next).ch;

            if is_digit(ch) || ch == '.' {
                number.push_char(ch)
            } else {
                break
            }

            *next = text.char_range_at(*next).next;
        }

        // convert the number string into a real number
        let n = match from_str::<f64>(number) {
            Some(n) => n,
            None    => {
                warn!(format!("warning: token.rs in token_number: couldn't convert {} into a floating point number!", number));
                return None
            }
        };

        return Some(Token{ id: Constant(Number(n)) });
    }

    fn iter(text: &str, op: &fn(c: char, next: &mut uint)) {

        let mut n = 0u;

        // Iterate through every character of the text an issue
        // the given closure on it.
        while n < text.len() {
            let ch = text.char_range_at(n).ch;
            let mut next = text.char_range_at(n).next;

            op(ch, &mut next);
            n = next
        }
    }

    pub fn each(t: &[Token], op: &fn(t: &Token)) {

        let mut n = 0u;

        // Iterate through every token and issue the given closure on it.
        while n < t.len() {
            op(&t[n]);
            n += 1;
        }
    }
}
