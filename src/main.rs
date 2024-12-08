use std::io::{self, Write};


mod in_to_pos {
    use std::{collections::LinkedList, fmt::{Debug, Display}, str::FromStr};
    use regex::Regex;

    #[derive(Debug)]
    pub enum PostfixError {
        ParseError,
        EmptyString
    }

    #[derive(Debug, PartialEq)]
    enum Tokens {
        Add,
        Sub,
        Mul,
        Div,
        Exp,
        OpP,
        ClP,
        Num(f64)
    }
    impl Display for Tokens {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Tokens::Add => write!(f, "+"),
                Tokens::Sub => write!(f, "-"),
                Tokens::Mul => write!(f, "*"),
                Tokens::Div => write!(f, "/"),
                Tokens::Exp => write!(f, "^"),
                Tokens::OpP => write!(f, "("),
                Tokens::ClP => write!(f, ")"),
                Tokens::Num(num) => write!(f, "[{}]", num),
            }
        }
    }
    impl FromStr for Tokens {
        type Err = PostfixError;
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(num) = s.parse::<f64>() {
                return Ok(Tokens::Num(num));
            } 
            
            match s {
                "+" => Ok(Tokens::Add),
                "-" => Ok(Tokens::Sub),
                "*" => Ok(Tokens::Mul),
                "/" => Ok(Tokens::Div),
                "^" => Ok(Tokens::Exp),
                "(" => Ok(Tokens::OpP),
                ")" => Ok(Tokens::ClP),
                _ => Err(PostfixError::ParseError)
            }
        }
    }
    impl Tokens {
        fn prio(&self) -> usize {
            match self {
                Tokens::Add => 1,
                Tokens::Sub => 1,
                Tokens::Mul => 2,
                Tokens::Div => 2,
                Tokens::Exp => 3,
                _ => 0,
            }
        }
    }
    
    pub struct Postfix {
        stack: LinkedList<Tokens>
    }

    impl Display for Postfix{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.stack.iter().try_for_each(|token| write!(f, "{}", token))
        }
    }

    impl Postfix {
        pub fn from_infix(infix: &str) -> Result<Postfix, PostfixError> {
            if infix.trim().is_empty() {
                return Err(PostfixError::EmptyString);
            }
        
            let re = Regex::new(r"\-?\d+(\.\d+)?|[\+\-\*/\^\(\)]").map_err(|_| PostfixError::ParseError)?;
            if re.split(infix)
                .any(|s| !s.trim().is_empty() && !s.trim().chars().all(|ch| ch.is_whitespace()))
            {
                return Err(PostfixError::ParseError);
            }
        
            let mut stack: Vec<Tokens> = Vec::new();
            let mut postfix = Postfix { stack: LinkedList::new() };
            let mut last_is_num = false;
        
            for token in re.find_iter(infix).map(|mat| mat.as_str()) {
                if let Ok(num) = token.parse::<f64>() {
                    postfix.stack.push_back(Tokens::Num(num));
                    last_is_num = true;
                } else {
                    match token {
                        "(" => {
                            if last_is_num {
                                stack.push(Tokens::Mul); // 2(5) can mean 2 * (5) 
                            }
                            stack.push(Tokens::OpP);
                        },
                        ")" => {
                            while let Some(top) = stack.pop() {
                                if matches!(top, Tokens::OpP) {
                                    break;
                                }
                                postfix.stack.push_back(top);
                            }
                        }
                        _ => {
                            let parsed_token: Tokens = token.parse().map_err(|_| PostfixError::ParseError)?;
                            while let Some(top) = stack.last() {
                                if parsed_token.prio() > top.prio() {
                                    break;
                                }
                                postfix.stack.push_back(stack.pop().unwrap());
                            }
                            stack.push(parsed_token);
                        }
                    }
                    last_is_num = false;
                }
            }
        
            while let Some(token) = stack.pop() {
                postfix.stack.push_back(token);
            }
        
            Ok(postfix)
        }  

        pub fn evaluate(&self) -> Result<f64, PostfixError> {
            let mut stack: Vec<f64> = Vec::new();

            for token in &self.stack {
                match token {
                    Tokens::Num(num) => stack.push(*num),
                    _ => {
                        let val1 = stack.pop().ok_or(PostfixError::ParseError)?;
                        let val2 = stack.pop().ok_or(PostfixError::ParseError)?;
                        match token {
                            Tokens::Add => stack.push(val2 + val1),
                            Tokens::Sub => stack.push(val2 - val1),
                            Tokens::Mul => stack.push(val2 * val1),
                            Tokens::Div => stack.push(val2 / val1),
                            Tokens::Exp => stack.push(val2.powf(val1)),
                            _ => {},
                        }
                    }
                }
            }

            stack.pop().ok_or(PostfixError::ParseError)
        }      
    }
}

fn parse_expression(expr: &str) {
    match in_to_pos::Postfix::from_infix(expr).and_then(|p| p.evaluate()) {
        Ok(result) => {println!("{expr} = {result}")},
        Err(err) => println!("Error: {err:?}"),
    }
}

fn main() {
    let mut input = String::new();
    println!("Input expression");
    print!(">> ");
    _ = io::stdout().flush();
    match io::stdin().read_line(&mut input).map(|_| input.trim()) {
        Ok(inp) => parse_expression(inp),
        Err(_) => println!("Input error..."),
    }
}
