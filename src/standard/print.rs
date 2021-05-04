use crate::engine::{function::Function, variable::Variable};

//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    print.rs
// Content: pmake standard print function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, error, variable};

pub struct Print {}

impl Function for Print {
    fn execute(
        &mut self,
        args: &Vec<variable::Variable>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError> {
        for arg in args {
            match &arg.typed {
                variable::VariableType::Boolean(bol) => {
                    println!("{}", bol)
                }

                variable::VariableType::Number(num) => {
                    println!("{}", num)
                }

                variable::VariableType::None => {
                    println!("{}", "None")
                }

                variable::VariableType::Str(str) => {
                    println!("{}", str.clone())
                }
            }
        }

        return Ok(Variable::none_value());
    }
}