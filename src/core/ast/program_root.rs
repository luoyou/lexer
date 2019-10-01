use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ProgramRoot {
    children : Vec<Box<AstNode>>
}


impl AstNode for ProgramRoot{

    /**
     * 从根节点进行求值
     */
    fn eval(&self, env: &mut Env)->Eval{
        let mut val = Eval::TNil;
        for child in &self.children{
            val = child.eval(env);
        }
        val
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
    
}

impl ProgramRoot{
    pub fn new()->Self{
        return ProgramRoot{
            children : vec![]
        }
    }

    pub fn push(&mut self, node: Box<AstNode>){
        self.children.push(node);
    }
}
