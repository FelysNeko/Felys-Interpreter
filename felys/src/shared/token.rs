pub enum KeywordType {
    While,
    If,
    Elif,
    Else,
    Return,
}

pub enum ValueType {
    Boolean,
    String,
    Number,
    None
}

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

pub enum UnaoptrType {
    Not,
    Pos,
    Neg
}

pub enum SymbolType {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicol,
    Comma
}

pub enum TokenType {
    Val(ValueType),
    Key(KeywordType),
    Bin(BinoptrType),
    Una(UnaoptrType),
    Sym(SymbolType),
    Identifier
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(t: TokenType, v: String) -> Self {
        Self { kind: t, value: v }
    }
}
