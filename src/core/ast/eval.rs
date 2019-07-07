use super::astree::Astree;

#[derive(Debug)]
pub enum BasicType{  // 基础类型
    TNil,
    TNumber,
    TString
}

#[derive(Debug)]
pub struct Eval {
    val_text: String, // 求值的文本内容
    val_type: BasicType
}

impl Eval{

    // 对节点进行求值
    pub fn new(ast: &Astree)->Self{
        let mut val = Eval{
            val_text: "".to_string(),
            val_type: BasicType::TNil,
        };
        val.getVal(ast);
        return val;
    }

    fn getVal(&mut self, ast: &Astree){
        self.val_type = BasicType::TNumber;
        if ast.is_leaf() {
            if ast.token().is_number() {
                self.val_text = ast.token().get_text().parse().unwrap();
            }else{
                panic!(ast.token().get_text().to_string() + " 类型错误");
            }
        }else{
            let left_ast = ast.children().get(0);
            let left = left_ast.unwrap().eval();
            
            let op_ast = ast.children().get(1);
            let op = op_ast.unwrap().token().get_text();

            let right_ast = ast.children().get(2);
            let right = right_ast.unwrap().eval();
            // println!("左值：{},右值：{}", left, right);
            match op {
                "+" => {
                    // self.val_text = left.val_text + right.val_text;
                    self.val_text = "5".to_string();
                },
                "-" => {
                    // self.val_text = left.val_text - right.val_text;
                    self.val_text = "5".to_string();
                },
                "*" => {
                    // self.val_text = left.val_text * right.val_text;
                    self.val_text = "5".to_string();
                },
                "/" => {
                    // self.val_text = left.val_text / right.val_text;
                    self.val_text = "5".to_string();
                },
                _ => panic!("操作符错误")
            }
        }
    }


}