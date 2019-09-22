use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;
use super::super::token::Token;

#[derive(Debug)]
pub struct IdentifierLeaf{
    token: Token
}


impl AstNode for IdentifierLeaf{
    
    fn eval(&mut self, env: &mut Env)->Eval{
        let val = env.get(&self.token.get_text().to_string());
        if val == Eval::TNil {
            panic!(self.token.get_text().to_string() + "变量未定义就直接使用");
        }else{
            return val;
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        panic!("标识符下没有子节点")
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

    fn get_id_name(&self)->String{
        return self.token.get_text().to_string();
    }
}

impl IdentifierLeaf{
    pub fn new(token: Token)->Self{
        IdentifierLeaf{
            token 
        }
    }
}
