use std::collections::HashMap;
use super::eval::Eval;

#[derive(Debug)]
pub struct Env{
    values: HashMap<String, Eval>
}

impl Env{
    pub fn new()->Self{
        Env{
            values: HashMap::new()
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
}