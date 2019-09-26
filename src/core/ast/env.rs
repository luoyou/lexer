use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::eval::Eval;
use super::ast_node::AstNode;
use super::super::system::sys_fn_map::{sys_fn_exist, sys_fn_call};

/**
 * 环境变量
 */
#[derive(Debug)]
pub struct Env{
    values: HashMap<String, Eval>,            // 全局作用域
    fn_map: HashMap<String, Rc<RefCell<Box<AstNode>>>>,    // 函数表
    fn_scopes: HashMap<String, HashMap<String, Eval>>, // String 由作用域中的函数名和key决定
    fn_stack: Vec<String>                     // 函数调用栈
}

impl Env{
    pub fn new()->Self{
        Env{
            values: HashMap::new(),
            fn_map: HashMap::new(),
            fn_scopes: HashMap::new(),
            fn_stack: Vec::new()
        }
    }

    pub fn put(&mut self, key: String, val: Eval){
        let len = self.fn_stack.len();
        if  len == 0 {
            self.values.insert(key, val);
        }else{
            let fn_name = &self.fn_stack[len - 1];
            if !self.fn_scopes.contains_key(fn_name) {
                self.fn_scopes.insert(fn_name.clone(), HashMap::new());
            }
            self.fn_scopes.get_mut(fn_name).unwrap().insert(key, val);
        }
    }

    pub fn get(&self, key: &String)->Eval{
        let len = self.fn_stack.len();
        let val: Option<&Eval>;
        if len == 0 {
            val = self.values.get(key);
        }else{
            let fn_name = &self.fn_stack[len - 1];
            val = self.fn_scopes[fn_name].get(key);
        }
        match val {
            Some(x) => x.clone(),
            None => Eval::TNil
        }
    }

    /**
     * 将函数节点推入环境变量中
     */
    pub fn put_fn(&mut self, fn_statement: Box<AstNode>){
        self.fn_map.insert(fn_statement.get_id_name(), Rc::new(RefCell::new(fn_statement)));        
    }

    /**
     * 进行函数调用
     */
    pub fn fn_call(&mut self, fn_name: String, param_list: &mut Vec<Eval>)->Eval{
        if sys_fn_exist(&*fn_name) {
            return sys_fn_call(fn_name, param_list);
            // let xxx = sys_fn_call(fn_name, param_list);
            // println!("{:#?}", xxx);            
            // return xxx;
        }else if self.fn_map.contains_key(&fn_name) { // 用户函数表查找
            self.fn_stack.push(fn_name.clone());
            let fn_node_rc = self.fn_map.get_mut(&fn_name).unwrap().clone();
            let mut fn_node = (*fn_node_rc).borrow_mut();
            let (params, block) = fn_node.call();
            for (index, param) in params.iter().enumerate() {
                // println!("{:#?}", index);
                self.put(param.get_id_name(), param_list.remove(0));
            }
            let val = block.eval(self);
            self.fn_stack.pop(); // 退出当前函数的作用域栈，todo 退出之后还要清空该函数产生的作用域
            return val;
        }else{
            panic!("函数".to_string() + &fn_name + "不存在")
        }
    }
}