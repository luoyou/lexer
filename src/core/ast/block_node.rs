use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct BlockNode {
    children: Vec<Box<AstNode>>
}


impl AstNode for BlockNode{

    fn eval(&self, env: &mut Env)->Eval{
        let mut val = Eval::TNil;
        // println!("{:#?}", self.children);
        for statement in &self.children {
            // println!("{:#?}", statement);
            // println!("+++++++++++++++++++++++++++++++++++");
            val = statement.eval(env);
            // println!("{:#?}", val);
            // println!("------------------------------------");

        }
        val
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}

impl BlockNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        return BlockNode{
            children: children
        }
    }
}
