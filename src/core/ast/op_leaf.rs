use super::astree::AstreeNode;
use super::eval::Eval;
use super::super::token::Token;
use super::env::Env;

#[derive(Debug)]
pub struct OpLeaf{
    token: Token
}

impl AstreeNode for OpLeaf{
    fn eval(&mut self, _: &mut Env)->Eval{
        return Eval::TText(self.token.get_text().to_string());
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        panic!("操作符下没有子节点")
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
impl OpLeaf{
    pub fn new(token: Token)->Self{
        OpLeaf{
            token
        }
    }
}
