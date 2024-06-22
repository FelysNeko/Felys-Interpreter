pub enum KeywordType {
    LET,
    WHILE,
    IF,
    ELIF,
    ELSE,
    RETURN,
}

pub enum ValueType {
    BOOLEAN,
    STRING,
    NUMBER,
    NONE
}

pub enum BinoptrType {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EQ,
    NE,
    LGR,
    SMR,
    LEQ,
    SEQ,
    ARR
}

pub enum UnaoptrType {
    NOT,
    POS,
    NEG
}

pub enum SymbolType {
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    SEMICOL,
    COMMA
}

pub enum TokenType {
    VAL(ValueType),
    KEY(KeywordType),
    BIN(BinoptrType),
    UNA(UnaoptrType),
    SYM(SymbolType),
    IDENTIFIER
}

pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub loc: (usize, usize)
}
