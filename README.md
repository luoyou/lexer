lexer 分词算法
---
## 符号系统
```
// 单行注释
/* 多行注释 */
 
' ' " "   『』「」   字符串定义，双引号进行转义，单引号不转义，可以交替使用，避免出错
{ }  代码块定义
( )  函数参数定义或确定运算的优先级
[ ]  数组的定义
.    有理数的小数点，对象调用函数，路径间隔符
+ - * / %   加减乘除余
=  赋值
> >= < <= ==  判断大小，必须为数字类型
:   定义字典
&   引用符
&&  且
||  或
!   取反

,   分隔符
\   转义符
->  定义函数返回值
```

## 关键词
```
if       若
else     否则
while    当
loop     循环
foreach  迭代
continue 跳过
break    终止

bool     逻辑
    true     真
    false    假
    and      且
    or       或
int      整数
float    有理数
string   文本
array    数组

let      令
class    模型
trait    特征
fn       函数
pub      公开
use      引入
return   返回
```

标识符：变量名，函数名，模型名

复合符号：
    - /* 注释开始 
    - */ 注释结束
整数类型：123, -21
有理数：1.872, -23.113
字符串："这是字符串", 
逻辑值：true, false

巴斯特范式，语法定义范式：

```
# 参照版本
primary   : "(" expr ")" | NUMBER | IDENTIFIER | STRING
factor    : "-" primary | primary
term      : factor { ("*" | "/" | "%") factor }
expr      :  term { ("+" | "-") term}
block     : "{" [statement] { (";" | EOL) [ statement ] } "}"
assign    : IDENTIFIER = expr
statement : "if" expr block [ "else" block]
            | "while" expr block
            | assign
program   : [statement] (";" | EOL)


# 渐进更新版本
program         : [statement] (";" | EOL)
statement       : expression | IDENTIFIER = expression | if_statement | while_statement
if_statement    : "if" expr block [ "else" block]
while_statement : "while" expr block
block           : "{" [statement] { (";" | EOL) [ statement ] } "}"
expression      : logical { ("&&" | "||") logical }
logical         : comparison { ("==" | ">" | ">=" | "<" | "<=") comparison }
comparison      : term   { ("+" | "-") term }
term            : factor { ("*" | "/" | "%") factor }
factor          : NUMBER | IDENTIFIER | BOOL | "(" expression ")" | - factor | ! factor

# 数字定义
数     -> 0|1|2|3|4|5|6|7|8|9
整数   -> 数 数*
有理数 -> 整数 | (整数'.'整数)
```
