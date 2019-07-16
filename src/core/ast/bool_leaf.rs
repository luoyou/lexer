use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;
use super::super::token::Token;

#[derive(Debug)]
pub struct BoolLeaf{
    token: Token
}


impl AstreeNode for BoolLeaf{
    
    fn eval(&self, env: &mut Env)->Eval{
        let val = self.token.get_text();
        if val == "true" || val == "真" {
            return Eval::TBool(true);
        }else{
            return Eval::TBool(false);
        }
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        panic!("逻辑值下没有子节点")
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

impl BoolLeaf{
    pub fn new(token: Token)->Self{
        BoolLeaf{
            token 
        }
    }
}
