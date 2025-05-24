enum Token {
    Number(f64),
    Operator { 
        op: char, 
        prec: u8, 
        right_assoc: bool,
        arity: usize
    },
    Function(String, usize),
    LParen,
    RParen,
    Coma 
}