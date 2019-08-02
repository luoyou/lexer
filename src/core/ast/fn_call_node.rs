use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 函数调用节点
 */
#[derive(Debug)]
pub struct FnCallNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for FnCallNode{

    fn eval(&mut self, env: &mut Env)->Eval{
        let fn_name = self.children.remove(0);
        let mut params: Vec<Eval> = Vec::new();
        for e in &mut self.children{
            params.push(e.eval(env));
        }
        env.fn_call(fn_name.get_id_name(), &mut params)
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }

    fn get_id_name(&self)->String{
        return self.children.get(0).unwrap().get_id_name();
    }
}

impl FnCallNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return FnCallNode{
            children: children
        }
    }
}
