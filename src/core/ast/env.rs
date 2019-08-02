use std::collections::HashMap;
use super::eval::Eval;
use super::astree::AstreeNode;

#[derive(Debug)]
pub struct Env{
    values: HashMap<String, Eval>,            // 全局作用域
    fn_map: HashMap<String, Box<AstreeNode>>, // 函数表
    fn_vals: HashMap<String, HashMap<String, Eval>>,
    scope: String
}

impl Env{
    pub fn new()->Self{
        Env{
            values: HashMap::new(),
            fn_map: HashMap::new(),
            fn_vals: HashMap::new(),
            scope: "".to_string()
        }
    }

    pub fn put(&mut self, key: String, val: Eval){
        self.values.insert(key, val);
    }

    pub fn get(&self, key: &String)->Eval{
        let val = self.values.get(key);
        match val {
            Some(x) => x.clone(),
            None => Eval::TNil
        }
    }

    /**
     * 将函数节点推入环境变量中
     */
    pub fn put_fn(&mut self, fn_statement: Box<AstreeNode>){
        self.fn_map.insert(fn_statement.get_id_name(), fn_statement);        
    }

    /**
     * 进行函数调用
     */
    pub fn fn_call(&mut self, fn_name: String, params: &mut Vec<Eval>)->Eval{
        if self.fn_map.contains_key(&fn_name) {
            Eval::TNil
        }else{
            panic!("函数".to_string() + &fn_name + "不存在")
        }
    }

    /**
     * 推入函数变量
     */
    pub fn put_fn_val(&mut self, fn_name: String, key: String, val: Eval){

    }

    /**
     * 读取函数内变量
     */
    pub fn get_fn_val(&mut self, fn_name: String, key: String)->Eval{
        Eval::TNil
    }

}