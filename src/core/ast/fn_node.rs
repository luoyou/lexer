use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 尝试不用children构建语法树，以简化语法树的节点数量
 * 函数节点的语法树在env环境中，而不在program语法树当中
 */
#[derive(Debug)]
pub struct FnNode {
    identifier: Box<AstreeNode>,
    param_list: Vec<Box<AstreeNode>>,
    block: Box<AstreeNode>
}


impl AstreeNode for FnNode{
    
    fn call(&mut self)->(Vec<Box<AstreeNode>>, Box<AstreeNode>){
        return (self.param_list, self.block);
    }

    /**
     * 因为该方法没有实现get_children, 所以需要实现 to_string
     */
    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        panic!("函数没有子类型")
    }

    fn get_id_name(&self)->String{
        self.identifier.get_id_name()
    }
}

impl FnNode{
    pub fn new(identifier: Box<AstreeNode>, param_list: Vec<Box<AstreeNode>>, block: Box<AstreeNode>)->Self{
        return FnNode{
            identifier,
            param_list,
            block
        }
    }
}
