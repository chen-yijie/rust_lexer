mod backend;
mod lexer;
mod parser;
use backend::*;
use lexer::*;
use parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file = env::args().last().unwrap();
    let file = "test.lexer";
    let source = std::fs::read_to_string( file )?;
    let lex = Lexer::new( source );
    let mut interp = Interpreter::new( lex );
    interp.execute();

    Ok( () )
}
