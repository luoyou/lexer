use std::collections::HashMap;
use super::eval::Eval;
use super::astree::AstreeNode;

#[derive(Debug)]
pub struct Env{
    values: HashMap<String, Eval>,            // 全局作用域
    fn_map: HashMap<String, Box<AstreeNode>>, // 函数表
    fn_vals: HashMap<String, HashMap<String, Eval>>, // String 由作用域中的函数名和key决定
    scope: Vec<String>
}

impl Env{
    pub fn new()->Self{
        Env{
            values: HashMap::new(),
            fn_map: HashMap::new(),
            fn_vals: HashMap::new(),
            scope: Vec::new()
        }
    }

    pub fn put(&mut self, key: String, val: Eval){
        let len = self.scope.len();
        if  len == 0 {
            self.values.insert(key, val);
        }else{
            let fn_name = &self.scope[len - 1];
            if !self.fn_vals.contains_key(fn_name) {
                self.fn_vals.insert(fn_name.clone(), HashMap::new());
            }
            self.fn_vals.get_mut(fn_name).unwrap().insert(key, val);
        }
    }

    pub fn get(&self, key: &String)->Eval{
        let len = self.scope.len();
        let val: Option<&Eval>;
        if len == 0 {
            val = self.values.get(key);
        }else{
            let fn_name = &self.scope[len - 1];
            val = self.fn_vals[fn_name].get(key);
        }
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
    pub fn fn_call(&mut self, fn_name: String, param_list: &mut Vec<Eval>)->Eval{
        if self.fn_map.contains_key(&fn_name) {
            self.scope.push(fn_name.clone());
            let (params, block) = self.fn_map.get_mut(&fn_name).unwrap().call();
            for (index, param) in params.iter().enumerate() {
                self.put(param.get_id_name(), param_list.remove(index));
            }
            Eval::TNil
        }else{
            panic!("函数".to_string() + &fn_name + "不存在")
        }
    }
}