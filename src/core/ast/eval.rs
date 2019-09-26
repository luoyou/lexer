
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

    pub fn connect(&self, right: &Eval)->Eval{
        let text = self.get_text_val();
        let mut text_str = text.to_string();
        text_str.push_str(right.get_text_val());
        Eval::TText(text_str)
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
            _ => panic!("该类型非逻辑值")
        }
    }

    fn get_text_val(&self)->&str{
        match self {
            Eval::TText(val) => val,
            _ => panic!("该类型不是文本值")
        }
    }

}