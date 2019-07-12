#[derive(Debug)]
pub enum Eval{  // 基础类型
    TNil,
    TNumber(i32),
    TString(String)
}

impl Eval{
    pub fn cal(&self, op: Eval, right: Eval)->Eval{
        let left_val  = match self {
            Eval::TNumber(val) => val,
            _ => panic!("数字出错")
        };
        let op_val = match op {
            Eval::TString(o) => o,
            _ => panic!("符号出错")
        };
        let right_val = match right {
            Eval::TNumber(val) => val,
            _ => panic!("数字出错")
        };
        match &*op_val {
            "+" => Eval::TNumber(left_val + right_val),
            "-" => Eval::TNumber(left_val - right_val),
            "*" => Eval::TNumber(left_val * right_val),
            "/" => Eval::TNumber(left_val / right_val),
            "%" => Eval::TNumber(left_val % right_val),
            _ => panic!("未支持的运算符")
        }
        
    }

}