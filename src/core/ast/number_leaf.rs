use super::astree::AstreeNode;
use super::eval::Eval;
use super::super::token::Token;

#[derive(Debug)]
pub struct NumberLeaf{
    token: Token
}


impl AstreeNode for NumberLeaf{
    fn eval(&self)->Eval{
        return Eval::TNumber(self.token.get_text().parse().unwrap());
    }

    fn location(&self)->String{
        let mut location = "位于第".to_string();
        if self.token.has_location(){
            location.push_str(&self.token.get_line_num().to_string());
            location.push('行');
        }else{
            location = "".to_string();
        }
        
        return location;
    }

    fn to_string(&self)->String{
        return self.token.get_text().to_string();
    }
}
impl NumberLeaf{
    pub fn new(token: Token)->Self{
        NumberLeaf{
            token 
        }
    }
}
