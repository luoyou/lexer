use super::ast_node::AstNode;
use super::eval::Eval;
use super::super::token::Token;
use super::env::Env;

#[derive(Debug)]
pub struct TextLeaf{
    token: Token
}


impl AstNode for TextLeaf{
    fn eval(&mut self, _: &mut Env)->Eval{
        return Eval::TText(self.token.get_text().to_string());
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        panic!("文本类型没有子节点")
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
impl TextLeaf{
    pub fn new(token: Token)->Self{
        TextLeaf{
            token 
        }
    }
}
