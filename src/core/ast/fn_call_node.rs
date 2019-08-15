use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 函数调用节点
 */
#[derive(Debug)]
pub struct FnCallNode {
    fn_name: Box<AstreeNode>,
    params: Vec<Box<AstreeNode>>
}


impl AstreeNode for FnCallNode{

    fn eval(&mut self, env: &mut Env)->Eval{
        let mut params: Vec<Eval> = Vec::new();
        for e in &mut self.params{
            params.push(e.eval(env));
        }
        env.fn_call(self.fn_name.get_id_name(), &mut params)
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        panic!("此节点暂不支持获取子节点");
    }

    fn get_id_name(&self)->String{
        return self.fn_name.get_id_name();
    }
}

impl FnCallNode{
    pub fn new(fn_name: Box<AstreeNode>, params: Vec<Box<AstreeNode>>)->Self{
        return FnCallNode{
            fn_name,
            params
        }
    }
}
