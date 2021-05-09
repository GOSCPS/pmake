//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    help.rs
// Content: pmake standard help function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, error, variable};
use crate::standard::help::error::RuntimeError;
use std::convert::TryInto;
use std::thread;
use std::time::Duration;

pub fn abort(
    _args: Vec<variable::Variable>,
    _: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
    Err(RuntimeError {
        reason_token: None,
        reason_err: None,
        reason_str: Some("Manual trigger -> abort() function.".to_string()),
        help_str: None,
        error_ast: None,
    })
}

// sleep
pub fn sleep(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> Result<variable::Variable, error::RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::create_error("sleep():Need one number arg!"));
    } else if let variable::VariableType::Number(num) = args[0].typed {
        thread::sleep(Duration::new(0, num.try_into().unwrap()));
        return Ok(variable::Variable::none_value());
    } else {
        return Err(RuntimeError::create_error("sleep():Need one number!"));
    }
}
