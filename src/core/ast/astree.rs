use super::super::token::Token;
use super::super::token::TokenType;

pub struct Astree {
    children_num: u8,
    token: Token
}

impl Astree {
    pub fn new()->Self{
        return Astree{
            children_num: 0,
            token: Token::new(0, "".to_string(), TokenType::End)
        }
    }

    fn child(&self, i: u8)->&Astree{
        if i == 0 {
            println!("aaa");
        }
        return self;
    }

    // fn children(&self)

    pub fn get_child_num(&self)->u8{
        return 0;
    }

    pub fn location(&self)->&str{
        return "位于";
    }

    // pub fn to_string()->
    
}