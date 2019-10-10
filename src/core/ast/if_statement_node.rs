use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct IfStatementNode {
    children: Vec<Box<AstNode>>
}


impl AstNode for IfStatementNode{

    /**
     */
    fn eval(&self, env: &mut Env)->Eval{
        let condition  = self.children[0].eval(env);
        if condition == Eval::TBool(true) {
            // println!("{:#?}", self.children[1]);
            // println!("===============================");
            self.children[1].eval(env)
        }else{
            if self.children.len() == 3 {
                self.children[2].eval(env)
            } else {
                Eval::TNil
            }
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}

impl IfStatementNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        return IfStatementNode{
            children: children
        }
    }
}
