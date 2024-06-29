#[derive(PartialEq, Debug)]
pub enum KeywordType {
    While,
    If,
    Elif,
    Else,
    Return,
}
pub type KT = KeywordType;


#[derive(PartialEq, Debug)]
pub enum ValueType {
    Boolean,
    String,
    Number,
    None
}
pub type VT = ValueType;


#[derive(PartialEq, Debug)]
pub enum BinoptrType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Ade,
    Sue,
    Mue,
    Die,
    Moe,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Xor,
    Or,
    Arr,
    Asn
}
pub type ST = SymbolType;

#[derive(PartialEq, Debug)]
pub enum UnaoptrType {
    Not,
    Pos,
    Neg
}
pub type BT = BinoptrType;


#[derive(PartialEq, Debug)]
pub enum SymbolType {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicol,
    Comma,
    Pipe
}
pub type UT = UnaoptrType;


#[derive(PartialEq, Debug)]
pub enum TokenType {
    Val(ValueType),
    Key(KeywordType),
    Bin(BinoptrType),
    Una(UnaoptrType),
    Sym(SymbolType),
    Identifier
}
pub type TT = TokenType;


#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}


impl Token {
    pub fn new(t: TokenType, v: String) -> Self {
        Self { kind: t, value: v }
    }
}
