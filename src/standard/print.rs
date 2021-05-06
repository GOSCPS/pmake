//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    print.rs
// Content: pmake standard print function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, error, variable, variable::Variable};

pub fn print(
    args: Vec<variable::Variable>,
<<<<<<< HEAD
    _context: &mut Context,
=======
    _: &mut Context,
>>>>>>> d1a03c05eb37a03255efa5244c3ee557d3837cae
) -> Result<variable::Variable, error::RuntimeError> {
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

<<<<<<< HEAD
    return Ok(Variable::none_value());
=======
    Ok(Variable::none_value())
>>>>>>> d1a03c05eb37a03255efa5244c3ee557d3837cae
}

pub fn println(
    args: Vec<variable::Variable>,
    context: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
<<<<<<< HEAD
    if let Err(err) = print(args, context) {
        return Err(err);
    }
    print!("\n");

    return Ok(Variable::none_value());
=======
    print(args, context)?;
    print!("\n");
    Ok(Variable::none_value())
}
pub fn eprint(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
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
    Ok(Variable::none_value())
}
pub fn eprintln(
    args: Vec<variable::Variable>,
    context: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
    eprint(args, context)?;
    eprint!("\n");
    Ok(Variable::none_value())
>>>>>>> d1a03c05eb37a03255efa5244c3ee557d3837cae
}
