use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ProgramRoot {
    children : Vec<Box<AstreeNode>>
}


impl AstreeNode for ProgramRoot{

    /**
     * 从根节点进行求值
     */
    fn eval(&mut self, env: &mut Env)->Eval{
        for child in &mut self.children{
            child.eval(env);
        } 
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
    
}

impl ProgramRoot{
    pub fn new()->Self{
        return ProgramRoot{
            children : vec![]
        }
    }

    pub fn push(&mut self, node: Box<AstreeNode>){
        self.children.push(node);
    }
}
