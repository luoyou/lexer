use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct WhileStatementNode {
    children: Vec<Box<AstNode>>
}


impl AstNode for WhileStatementNode{

    /**
     *
     */
    fn eval(&self, env: &mut Env)->Eval{
        loop {
            if self.children[0].eval(env) == Eval::TBool(true) {
                self.children[1].eval(env);
            }else{
                break;
            }
        }
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}

impl WhileStatementNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        return WhileStatementNode{
            children: children
        }
    }
}
