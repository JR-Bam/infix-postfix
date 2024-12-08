use std::{collections::LinkedList, fmt::{Debug, Display}, str::FromStr};
use regex::Regex;

#[derive(Debug, PartialEq)]
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
                        let mut found_open_paren = false;
                        while let Some(top) = stack.pop() {
                            if matches!(top, Tokens::OpP) {
                                found_open_paren = true;
                                break;
                            }
                            postfix.stack.push_back(top);
                        }
                        if !found_open_paren {
                            return Err(PostfixError::ParseError);
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


pub fn parse_expression(expr: &str) {
    match Postfix::from_infix(expr).and_then(|p| p.evaluate()) {
        Ok(result) => {println!("{expr} = {result}")},
        Err(err) => println!("Error: {err:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_postfix_result(infix: &str, expected_postfix: &str, expected_value: Option<f64>) {
        match Postfix::from_infix(infix) {
            Ok(postfix) => {
                assert_eq!(postfix.to_string(), expected_postfix, "Postfix conversion failed for '{}'", infix);
                if let Some(expected) = expected_value {
                    let eval_result = postfix.evaluate();
                    assert!(eval_result.is_ok(), "Evaluation failed for '{}'", infix);
                    assert!((eval_result.unwrap() - expected).abs() < 1e-9, "Evaluation result mismatch for '{}'", infix);
                }
            }
            Err(err) => panic!("Unexpected error for '{}': {:?}", infix, err),
        }
    }

    fn assert_postfix_error(infix: &str, expected_error: PostfixError) {
        match Postfix::from_infix(infix).and_then(|pf| pf.evaluate()) {
            Ok(_) => panic!("Expected error for '{}', but got a valid postfix expression", infix),
            Err(err) => assert_eq!(err, expected_error),
        }
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_postfix_result("2 + 3", "[2][3]+", Some(5.0));
        assert_postfix_result("5 - 3", "[5][3]-", Some(2.0));
        assert_postfix_result("4 * 3", "[4][3]*", Some(12.0));
        assert_postfix_result("6 / 3", "[6][3]/", Some(2.0));
        assert_postfix_result("2 ^ 3", "[2][3]^", Some(8.0));
    }

    #[test]
    fn test_complex_expressions() {
        assert_postfix_result("2 + 3 * 4", "[2][3][4]*+", Some(14.0));
        assert_postfix_result("(2 + 3) * 4", "[2][3]+[4]*", Some(20.0));
        assert_postfix_result("((2 + 3) * (4 - 1)) / 3", "[2][3]+[4][1]-*[3]/", Some(5.0));
    }

    #[test]
    fn test_edge_cases() {
        assert_postfix_result("2(3 + 4)", "[2][3][4]+*", Some(14.0));
        assert_postfix_result("-2 + 3", "[-2][3]+", Some(1.0));
        assert_postfix_result("-3^2", "[-3][2]^", Some(9.0));
        assert_postfix_error("", PostfixError::EmptyString);
        assert_postfix_error("2 + a", PostfixError::ParseError);
    }

    #[test]
    fn test_floating_point() {
        assert_postfix_result("2.5 * 4", "[2.5][4]*", Some(10.0));
        assert_postfix_result("3 + 2.5 * 4", "[3][2.5][4]*+", Some(13.0));
        assert_postfix_result("10 / 4", "[10][4]/", Some(2.5));
    }

    #[test]
    fn test_whitespace_handling() {
        assert_postfix_result("  2   +   3   ", "[2][3]+", Some(5.0));
        assert_postfix_result("2\n+\t3", "[2][3]+", Some(5.0));
    }

    #[test]
    fn test_operator_precedence() {
        assert_postfix_result("5 - 2 + 1", "[5][2]-[1]+", Some(4.0));
        assert_postfix_result("2 ^ 3 ^ 2", "[2][3]^[2]^", Some(64.0));
    }

    #[test]
    fn test_error_cases() {
        assert_postfix_error("(2 + 3", PostfixError::ParseError);
        assert_postfix_error("2 + 3)", PostfixError::ParseError);
        assert_postfix_error("2 +", PostfixError::ParseError);
    }
}
