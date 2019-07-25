use std::collections::HashMap;
use super::astree::AstreeNode;

/**
 * 函数名表，用于查表调用函数
 */
#[derive(Debug)]
pub struct FnMap{
    values: HashMap<String, Box<AstreeNode>>
}

impl FnMap{
    pub fn new()->Self{
        FnMap{
            values: HashMap::new()
        }
    }

    pub fn put(&mut self, val: Box<AstreeNode>){
        self.values.insert(val.get_id_name(), val);
    }
}