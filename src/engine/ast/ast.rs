//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    ast.rs
// Content: pmake ast source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{
    context::Context,
    error::{self, RuntimeError},
    variable::{self, Variable, VariableType},
};
use std::sync::Arc;

// 抽象语法树
pub trait Ast {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError>;
}

pub struct NopAst {}

impl Ast for NopAst {
    fn execute(&self, _context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        Ok(Variable {
            name: Arc::from("# Temporary value from NopAST #".to_string()),
            typed: variable::VariableType::None,
        })
    }
}

pub struct AssignmentAst {
    pub global: bool,
    pub name: String,
    pub value: Box<dyn Ast>,
}

impl Ast for AssignmentAst {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        match self.value.execute(context) {
            Ok(ok) => {
                // 全局变量
                if self.global {
                    crate::engine::context::GLOBAL_CONTEXT
                        .variable_table
                        .write()
                        .unwrap()
                        .insert(self.name.clone(), ok);
                }
                // 本地变量
                else {
                    context
                        .variable_table
                        .write()
                        .unwrap()
                        .insert(self.name.clone(), ok);
                }
                Ok(Variable::none_value())
            }

            Err(err) => Err(err),
        }
    }
}

pub struct ImmediateAst {
    pub immediate: Variable,
}

impl Ast for ImmediateAst {
    fn execute(&self, _context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        return Ok(self.immediate.clone());
    }
}

pub struct BlockAst {
    pub blocks: Vec<Box<dyn Ast>>,
}

impl Ast for BlockAst {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        let mut var: Variable = Variable::none_value();

        for ast in &self.blocks {
            match ast.execute(context) {
                Ok(output) => var = output,

                Err(err) => return Err(err),
            }
        }

        return Ok(var);
    }
}

// 获取变量Ast
pub struct GetVariableAst {
    pub name: String,
}

impl Ast for GetVariableAst {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        // 从本地变量获取
        if context
            .variable_table
            .read()
            .unwrap()
            .contains_key(&self.name)
        {
            return Ok(context
                .variable_table
                .read()
                .unwrap()
                .get(&self.name)
                .unwrap()
                .clone());
        }
        // 本地变量未找到
        // 从全局变量获取
        else if crate::engine::context::GLOBAL_CONTEXT
            .variable_table
            .read()
            .unwrap()
            .contains_key(&self.name)
        {
            return Ok(crate::engine::context::GLOBAL_CONTEXT
                .variable_table
                .read()
                .unwrap()
                .get(&self.name)
                .unwrap()
                .clone());
        }
        // 未找到变量
        else {
            return Err(RuntimeError {
                reason_token: None,
                reason_err: None,
                reason_str: Some(format!("The variable `{}` not found!", self.name)),
                help_str: Some("Check the variable name!".to_string()),
            });
        }
    }
}

#[derive(std::cmp::PartialEq)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
    Left,
    Right,
    Pipeline,
}

// 表达式AST
pub struct ExprAst {
    pub left: Box<dyn Ast>,
    pub right: Box<dyn Ast>,
    pub op: ExprOp,
}

impl Ast for ExprAst {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        // 特殊的操作
        if let ExprOp::Left = self.op {
            return self.left.execute(context);
        } else if let ExprOp::Right = self.op {
            return self.right.execute(context);
        } else if let ExprOp::Pipeline = self.op {
            let lft = self.left.execute(context);

            if let Result::Err(err) = lft {
                return Err(err);
            } else {
                return self.right.execute(context);
            }
        }

        // 准备好左值和右值
        let left_ = self.left.execute(context);
        let right_;

        if left_.is_err() {
            return Err(left_.err().unwrap());
        } else {
            right_ = self.right.execute(context);

            if right_.is_err() {
                return Err(right_.err().unwrap());
            }
        }

        let left = left_.unwrap();
        let right = right_.unwrap();

        // 检查类型
        if left.typed != right.typed {
            return Err(RuntimeError {
                reason_token: None,
                reason_err: None,
                reason_str: Some(format!(
                    "The type of variable `{}` and `{}` are not some!",
                    &left.name, &right.name
                )),
                help_str: Some("Check the variable name and type!".to_string()),
            });
        }

        // None不进行计算
        if left.typed == VariableType::None {
            return Err(RuntimeError {
                reason_token: None,
                reason_err: None,
                reason_str: Some(format!(
                    "The None variable `{}` and `{}` cannot operating!",
                    &left.name, &right.name
                )),
                help_str: Some("Check the variable name and type!".to_string()),
            });
        }

        match left.typed {
            // 字符串
            VariableType::Str(str) => {
                // 只允许+
                match self.op {
                    ExprOp::Add => {
                        if let VariableType::Str(rgt) = right.typed {
                            return Ok(Variable {
                                name: Arc::from("# Expr Ast #"),
                                typed: VariableType::Str(str + &*rgt),
                            });
                        } else {
                            unreachable!("type of LFT not equals to RGH!");
                        }
                    }

                    _ => {
                        return Err(RuntimeError {
                            reason_token: None,
                            reason_err: None,
                            reason_str: Some(format!(
                                "The string variable `{}` and `{}` cannot operating except +!",
                                &left.name, &right.name
                            )),
                            help_str: Some(
                                "Check the variable name and type and operating type!".to_string(),
                            ),
                        });
                    }
                }
            }

            // 数字
            VariableType::Number(num) => {
                if let VariableType::Number(rgt) = right.typed {
                    match self.op {
                        // 检查运算
                        ExprOp::Add => {
                            return Ok(Variable {
                                name: Arc::from("# Expr Ast #"),
                                typed: VariableType::Number(num + rgt),
                            });
                        }

                        ExprOp::Sub => {
                            return Ok(Variable {
                                name: Arc::from("# Expr Ast #"),
                                typed: VariableType::Number(num - rgt),
                            });
                        }

                        ExprOp::Mul => {
                            return Ok(Variable {
                                name: Arc::from("# Expr Ast #"),
                                typed: VariableType::Number(num * rgt),
                            });
                        }

                        ExprOp::Div => {
                            return Ok(Variable {
                                name: Arc::from("# Expr Ast #"),
                                typed: VariableType::Number(num / rgt),
                            });
                        }

                        _ => {
                            unreachable!("UNKNOWN OP AST TYPE!");
                        }
                    }
                } else {
                    unreachable!("type of LFT not equals to RGH!");
                }
            }

            _ => {
                unreachable!("UNKNOWN OP AST TYPE!");
            }
        }
    }
}

// 函数调用Ast
pub struct CallAst {
    pub name: String,
    pub args: Vec<Box<dyn Ast>>,
}

impl Ast for CallAst {
    fn execute(&self, context: &mut Context) -> Result<variable::Variable, error::RuntimeError> {
        let lock = super::super::context::GLOBAL_FUNCTION.lock().unwrap();

        match lock.get(&self.name) {
            Some(some) => {
                let mut arg_value: Vec<Variable> = Vec::new();

                let func = *some;

                drop(lock);

                // 检索参数
                for arg in self.args.iter() {
                    match (**arg).execute(context) {
                        Err(err) => return Err(err),

                        Ok(ok) => arg_value.push(ok),
                    }
                }

                return func(arg_value, context);
            }

            None => {
                return Err(RuntimeError {
                    reason_token: None,
                    reason_err: None,
                    reason_str: Some(format!("The function `{}` not found!", self.name)),
                    help_str: Some("Check the function name!".to_string()),
                })
            }
        }
    }
}
