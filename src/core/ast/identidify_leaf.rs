use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;
use super::super::token::Token;

#[derive(Debug)]
pub struct IdentidifyLeaf{
    token: Token
}


impl AstreeNode for IdentidifyLeaf{
    
    fn eval(&self, env: &mut Env)->Eval{
        return env.get(&self.token.get_text().to_string());
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
impl IdentidifyLeaf{
    pub fn new(token: Token)->Self{
        IdentidifyLeaf{
            token 
        }
    }
}
