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
use std::collections::VecDeque;
use std::io::Read;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::path::PathBuf;
use std::process;
use std::process::Stdio;
use std::sync::Arc;
use std::thread;

// Ast执行结果
pub enum AstResult{
    Ok(variable::Variable),
    Err(error::RuntimeError),
    // 中断
    // 不再执行
    Interrupt
}

impl AstResult{
    pub fn unwrap(&self) -> variable::Variable{
        if let Ok(ok) = self{
            return ok.clone();
        }
        else{
            panic!("The AstResult isn't ok!");
        }
    }
}

use AstResult::Ok;
use AstResult::Err;
use AstResult::Interrupt;

// 抽象语法树
pub trait Ast: Send + Sync {
    fn execute(&self, context: &mut Context) -> AstResult;
    fn clone(&self) -> Box<dyn Ast>;

    // return (文件名称,行号)
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)>;
}

impl Clone for Box<dyn Ast> {
    fn clone(&self) -> Self {
        Ast::clone(&**self)
    }
}

#[derive(Clone)]
pub struct NopAst {
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for NopAst {
    fn execute(&self, _context: &mut Context) -> AstResult {
        std::hint::spin_loop();
        Ok(Variable {
            name: Arc::from("# Temporary value from NopAST #".to_string()),
            typed: variable::VariableType::None,
        })
    }
    fn clone(&self) -> Box<dyn Ast> {
        Box::new(NopAst {
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

#[derive(Clone)]
pub struct AssignmentAst {
    pub global: bool,
    pub name: String,
    pub value: Box<dyn Ast>,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for AssignmentAst {
    fn execute(&self, context: &mut Context) -> AstResult {
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

            Interrupt => Interrupt
        }
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(AssignmentAst {
            global: self.global,
            name: self.name.clone(),
            value: self.value.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}
#[derive(Clone)]
pub struct ImmediateAst {
    pub immediate: Variable,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for ImmediateAst {
    fn execute(&self, _context: &mut Context) -> AstResult {
        return Ok(self.immediate.clone());
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(ImmediateAst {
            immediate: self.immediate.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

#[derive(Clone)]
pub struct BlockAst {
    pub blocks: Vec<Box<dyn Ast>>,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for BlockAst {
    fn execute(&self, context: &mut Context) -> AstResult {
        let mut var: Variable = Variable::none_value();

        for ast in &self.blocks {
            match ast.execute(context) {
                Ok(output) => var = output,

                Err(err) => return Err(err),

                // 中断
                // 不再执行
                Interrupt => break
            }
        }

        return Ok(var);
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(BlockAst {
            blocks: self.blocks.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// 获取变量Ast
#[derive(Clone)]
pub struct GetVariableAst {
    pub name: String,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for GetVariableAst {
    fn clone(&self) -> Box<dyn Ast> {
        Box::new(GetVariableAst {
            name: self.name.clone(),
            position: self.position.clone(),
        })
    }

    fn execute(&self, context: &mut Context) -> AstResult {
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
                error_ast: Some(Box::new(Clone::clone(self))),
            });
        }
    }

    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

#[derive(std::cmp::PartialEq, Clone, Copy)]
pub enum ExprOp {
    Add,
    Sub,
    Mul,
    Div,
    Left,
    //Right,
    //Pipeline,
}

// 表达式AST
#[derive(Clone)]
pub struct ExprAst {
    pub left: Box<dyn Ast>,
    pub right: Box<dyn Ast>,
    pub op: ExprOp,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for ExprAst {
    fn execute(&self, context: &mut Context) -> AstResult {
        // 特殊的操作
        if let ExprOp::Left = self.op {
            return self.left.execute(context);
        } /*else if let ExprOp::Right = self.op {
            return self.right.execute(context);
        } else if let ExprOp::Pipeline = self.op {
            let lft = self.left.execute(context);

            if let Err(err) = lft {
                return Err(err);
            } else {
                return self.right.execute(context);
            }
        }*/

        // 准备好左值和右值
        let left_ = self.left.execute(context);
        let right_;

        if let Err(err) = left_ {
            return Err(err);
        } else {
            right_ = self.right.execute(context);

            if let Err(err) = right_ {
                return Err(err);
            }
        }

        let left = left_.unwrap();
        let right = right_.unwrap();

        // 检查类型
        /*if left.typed != right.typed {
            return Err(RuntimeError {
                reason_token: None,
                reason_err: None,
                reason_str: Some(format!(
                    "The type of variable `{}` and `{}` are not some!",
                    &left.name, &right.name
                )),
                help_str: Some("Check the variable name and type!".to_string()),
            });
        }*/

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
                error_ast: Some(Box::new(Clone::clone(self))),
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
                                name: Arc::from("# Expr Ast For Append String#"),
                                typed: VariableType::Str(str + &*rgt),
                            });
                        } else {
                            return Err(RuntimeError {
                                reason_token: None,
                                reason_err: None,
                                reason_str: Some(format!(
                                    "The type of variable `{}` and `{}` are not some!",
                                    &left.name, &right.name
                                )),
                                help_str: Some("Check the variable name and type!".to_string()),
                                error_ast: Some(Box::new(Clone::clone(self))),
                            });
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
                            error_ast: Some(Box::new(Clone::clone(self))),
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
                    return Err(RuntimeError {
                        reason_token: None,
                        reason_err: None,
                        reason_str: Some(format!(
                            "The type of variable `{}` and `{}` are not some!",
                            &left.name, &right.name
                        )),
                        help_str: Some("Check the variable name and type!".to_string()),
                        error_ast: Some(Box::new(Clone::clone(self))),
                    });
                }
            }

            _ => {
                unreachable!("UNKNOWN OP AST TYPE!");
            }
        }
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(ExprAst {
            left: self.left.clone(),
            right: self.right.clone(),
            op: self.op,
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// 函数调用Ast
#[derive(Clone)]
pub struct CallAst {
    pub name: String,
    pub args: Vec<Box<dyn Ast>>,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for CallAst {
    fn execute(&self, context: &mut Context) -> AstResult {
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

                        Interrupt => return Interrupt
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
                    error_ast: Some(Box::new(Clone::clone(self))),
                })
            }
        }
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(CallAst {
            name: self.name.clone(),
            args: self.args.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// try ast
#[derive(Clone)]
pub struct TryAst {
    pub aim: Box<dyn Ast>,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for TryAst {
    fn execute(&self, _context: &mut Context) -> AstResult {
        let wrapper = AssertUnwindSafe(&self);

        let result = panic::catch_unwind(move || match wrapper.aim.execute(&mut Context::new()) {
            Err(err) => Err(err),

            Ok(ok) => Ok(ok),

            Interrupt => Interrupt
        });

        if result.is_err() {
            crate::tool::printer::debug_line(&format!(
                "{}:Try err - panic!",
                thread::current().name().unwrap_or("UNKNOWN")
            ));

            return Ok(Variable::none_value());
        } else if let Some(some) = result.ok() {
            return match some {
                Err(err) => {
                    crate::tool::printer::debug_line(&format!(
                        "{}:Try err:{}",
                        thread::current().name().unwrap_or("UNKNOWN"),
                        err
                    ));

                    Ok(Variable::none_value())
                }

                Ok(ok) => Ok(ok),

                Interrupt => Interrupt
            };
        }
        // 不可能
        else {
            unreachable!("Unreachable:RESULT isn't err and ok!");
        }
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(TryAst {
            aim: self.aim.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// shell execute ast
#[derive(Clone)]
pub struct ShAst {
    pub args: Vec<Box<dyn Ast>>,
    pub position: Option<(Arc<PathBuf>, usize)>,
    pub output : bool
}

impl Ast for ShAst {
    fn execute(&self, context: &mut Context) -> AstResult {
        // 获取参数
        let mut variable: VecDeque<Variable> = VecDeque::new();

        for arg in &self.args {
            match arg.execute(context) {
                Err(err) => return Err(err),

                Ok(ok) => variable.push_back(ok),

                Interrupt => return Interrupt
            }
        }

        if variable.len() == 0 {
            return Err(RuntimeError::create_error(
                "Need one arg to as program name as least!",
            ));
        }

        // 命令
        let mut cmd_str = String::new();

        // 执行
        let temp = variable.pop_front().unwrap();
        cmd_str.push_str(&temp.to_string());
        cmd_str.push(' ');

        let mut cmd = process::Command::new(temp.to_string());

        for arg in variable.into_iter() {
            cmd_str.push_str(&arg.to_string());
            cmd_str.push(' ');
            cmd.arg(arg.to_string());
        }

        // 输出命令
        crate::tool::printer::trace_line(&format!(
            "{}:{}",
            thread::current().name().unwrap_or("UNKNOWN"),
            cmd_str
        ));

        // 执行
        return match cmd.stdout(Stdio::piped()).spawn() {
            Result::Err(err) => Err(RuntimeError::create_error(&err.to_string())),

            Result::Ok(mut ok) => {
                ok.wait().unwrap();
                // 退出代码
                let code = cmd.status().unwrap().code().unwrap_or(1);

                let mut output = String::new();
                ok.stdout.unwrap().read_to_string(&mut output).unwrap();

                // 输出output
                if self.output{
                if output.ends_with('\n') {
                    crate::tool::printer::write(&format!("{}", output));
                } else {
                    crate::tool::printer::write(&format!("{}\n", output));
                }
                }

                // 非0
                if code != 0 {
                    return Err(RuntimeError::create_error(
                        "The exec return code isn't zero!",
                    ));
                }

                Ok(Variable {
                    name: Arc::from("# ShAst program output #"),
                    typed: VariableType::Str(output),
                })
            }
        };
    }

    fn clone(&self) -> Box<dyn Ast> {
        Box::new(ShAst {
            args: self.args.clone(),
            position: self.position.clone(),
            output : self.output
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// 返回Ast
#[derive(Clone)]
pub struct ReturnAst {
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for ReturnAst {
    fn execute(&self, _context: &mut Context) -> AstResult {
        AstResult::Interrupt
    }
    fn clone(&self) -> Box<dyn Ast> {
        Box::new(ReturnAst {
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}

// 判断true or false
pub fn condition(var : &Variable) -> bool{
    match &var.typed{
        VariableType::None => false,

        VariableType::Boolean(boolean) => *boolean,

        VariableType::Number(num) => if *num == 0 {false} else {true},

        VariableType::Str(strs) => if strs.len() == 0{false} else {true}
    }
}

// if ast
#[derive(Clone)]
pub struct IfAst {
    pub condition : Box<dyn Ast>,
    pub body : Box<dyn Ast>,
    pub also : Box<dyn Ast>,
    pub position: Option<(Arc<PathBuf>, usize)>,
}

impl Ast for IfAst {
    fn execute(&self, context: &mut Context) -> AstResult {
        // 检查条件
        match self.condition.execute(context){
            Err(err) => return Err(err),

            // 执行
            Ok(ok) =>{
                return if condition(&ok) {
                    self.body.execute(context)
                }
                else{
                    self.also.execute(context)
                }
            }

            Interrupt => return Interrupt
        }
    }
    fn clone(&self) -> Box<dyn Ast> {
        Box::new(IfAst {
            condition : self.condition.clone(),
            body : self.body.clone(),
            also : self.also.clone(),
            position: self.position.clone(),
        })
    }
    fn get_position(&self) -> Option<(Arc<PathBuf>, usize)> {
        return self.position.clone();
    }
}