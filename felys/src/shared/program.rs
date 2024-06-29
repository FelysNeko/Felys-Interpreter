use crate::shared::statement::Statement;
use crate::shared::token::Token;

pub struct Program {
    pub tokens: Vec<Token>,
    pub worker: Option<Statement>
}
