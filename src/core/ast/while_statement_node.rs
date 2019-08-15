use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct WhileStatementNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for WhileStatementNode{

    /**
     *
     */
    fn eval(&mut self, env: &mut Env)->Eval{
        loop {
            if self.children[0].eval(env) == Eval::TBool(true) {
                self.children[1].eval(env);
            }else{
                break;
            }
        }
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl WhileStatementNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return WhileStatementNode{
            children: children
        }
    }
}
