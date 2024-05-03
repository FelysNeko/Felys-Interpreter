pub mod frontend;
pub mod runtime;


use self::frontend::Statement;

#[derive(Debug)]
pub struct Program {
    pub body: Vec<Statement>
}