//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    print.rs
// Content: pmake standard print function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, variable, variable::Variable,ast::ast::AstResult};

pub fn print(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    for arg in args {
        match &arg.typed {
            variable::VariableType::Boolean(bol) => {
                print!("{}", bol)
            }

            variable::VariableType::Number(num) => {
                print!("{}", num)
            }

            variable::VariableType::None => {
                print!("{}", "`NONE`")
            }

            variable::VariableType::Str(str) => {
                print!("{}", str.clone())
            }
        }
    }
    AstResult::Ok(Variable::none_value())
}

pub fn println(
    args: Vec<variable::Variable>,
    context: &mut Context,
) -> AstResult {
    match print(args, context){
        AstResult::Err(err) => return AstResult::Err(err),

        AstResult::Ok(_ok) => {
            print!("\n");
            return AstResult::Ok(Variable::none_value());
        }

        AstResult::Interrupt => {
            return AstResult::Ok(Variable::none_value());
        }
    }
}

pub fn eprint(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    for arg in args {
        match &arg.typed {
            variable::VariableType::Boolean(bol) => {
                eprint!("{}", bol)
            }

            variable::VariableType::Number(num) => {
                eprint!("{}", num)
            }

            variable::VariableType::None => {
                eprint!("{}", "`NONE`")
            }

            variable::VariableType::Str(str) => {
                eprint!("{}", str.clone())
            }
        }
    }
    AstResult::Ok(Variable::none_value())
}

pub fn eprintln(
    args: Vec<variable::Variable>,
    context: &mut Context,
) -> AstResult {
    match eprint(args, context){
        AstResult::Err(err) => return AstResult::Err(err),

        AstResult::Ok(_ok) => {
            eprint!("\n");
            return AstResult::Ok(Variable::none_value());
        }

        AstResult::Interrupt => {
            return AstResult::Ok(Variable::none_value());
        }
    }
}
