#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
    NULL,
    IDENT,
    NUMBER,
    STRING,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    SEMICOL,

    // keyword
    WHILE,
    IF,
    ELIF,
    ELSE,

    // boolean
    TRUE,
    FALSE,

    // binary operator
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    LGR,
    SMR,
    LEQ,
    SEQ,
    ASN,
    EQ,
    NE,
    AND,
    OR,

    // unary operator
    POS,
    NEG,
    NOT,
}