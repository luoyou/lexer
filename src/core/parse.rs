use super::lexer::Lexer;

pub struct Parse{
    lexer: Lexer
}

impl Parse{
    pub fn new(lexer: Lexer)->Self{
        return Parse{
            lexer: lexer
        }
    }

    
}