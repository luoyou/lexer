
#[derive(Debug, Clone, PartialEq)]
pub enum Eval{  // 基础类型
    TNil,
    TBool(bool),
    TNumber(i32),
    TText(String)
}

impl Eval{
    /**
     * 数值计算
     */
    pub fn cal(&self, left: Eval, right: Eval)->Eval{
        let op_val = match self {
            Eval::TText(o) => o,
            _ => panic!("符号出错")
        };
        match &**op_val {
            "+"  => Eval::TNumber(left.get_number_val() + right.get_number_val()),
            "-"  => Eval::TNumber(left.get_number_val() - right.get_number_val()),
            "*"  => Eval::TNumber(left.get_number_val() * right.get_number_val()),
            "/"  => Eval::TNumber(left.get_number_val() / right.get_number_val()),
            "%"  => Eval::TNumber(left.get_number_val() % right.get_number_val()),
            "==" => Eval::TBool(left == right),
            ">"  => Eval::TBool(left.get_number_val() > right.get_number_val()),
            ">=" => Eval::TBool(left.get_number_val() >= right.get_number_val()),
            "<"  => Eval::TBool(left.get_number_val() < right.get_number_val()),
            "<=" => Eval::TBool(left.get_number_val() <= right.get_number_val()),
            "&&" => Eval::TBool(left.get_bool_val() && right.get_bool_val()),
            "||" => Eval::TBool(left.get_bool_val() || right.get_bool_val()),
            _ => panic!("未支持的运算符")
        }
    }

    fn get_number_val(&self)->&i32{
        match self {
            Eval::TNumber(val) => val,
            _ => panic!("数值错误")
        }
    }

    fn get_bool_val(&self)->bool{
        match self {
            Eval::TBool(val) => *val,
            _ => panic!("数值错误")
        }
    }

}