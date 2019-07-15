use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct BlockNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for BlockNode{

    fn eval(&self, env: &mut Env)->Eval{
        for statement in self.get_children() {
            statement.eval(env);
        }
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl BlockNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return BlockNode{
            children: children
        }
    }
}
