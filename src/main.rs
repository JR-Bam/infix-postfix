use std::io::{self, Write};


mod exprparse;
fn main() {
    let mut input = String::new();
    println!("Input expression");
    print!(">> ");
    _ = io::stdout().flush();
    match io::stdin().read_line(&mut input).map(|_| input.trim()) {
        Ok(inp) => exprparse::parse_expression(inp),
        Err(_) => println!("Input error..."),
    }
}