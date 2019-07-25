use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;
use super::fn_map::FnMap;

#[derive(Debug)]
pub struct ProgramRoot {
    children : Vec<Box<AstreeNode>>,
    fn_map : FnMap
}


impl AstreeNode for ProgramRoot{

    /**
     * 从根节点进行求值
     */
    fn eval(&self, env: &mut Env)->Eval{
        for child in &self.children{
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
            children : vec![],
            fn_map : FnMap::new()
        }
    }

    pub fn push_statement(&mut self, node: Box<AstreeNode>){
        self.children.push(node);
    }

    pub fn put_fn(&mut self, node: Box<AstreeNode>){
        self.fn_map.put(node);
    }
}
