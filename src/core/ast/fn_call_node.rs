use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct FnCallNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for FnCallNode{

    fn eval(&self, env: &mut Env)->Eval{
        Eval::TNil
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl FnCallNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return FnCallNode{
            children: children
        }
    }
}
