//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    help.rs
// Content: pmake standard help function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context,variable,ast::ast::AstResult};
use std::convert::TryInto;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use crate::engine::error::RuntimeError;

pub fn abort(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    let mut reason = String::new();

    for arg in args.into_iter(){
        reason.push_str(&arg.to_string());
        reason.push(' ');
    }

    if reason.len() == 0{
        reason.push_str("Interrupt");
    }

    AstResult::Err(RuntimeError {
        reason_token: None,
        reason_err: None,
        reason_str: Some(format!("abort():{}",reason)),
        help_str: None,
        error_ast: None,
    })
}

// sleep
pub fn sleep(
    args: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    if args.len() != 1 {
        return AstResult::Err(RuntimeError::create_error("sleep():Need one number arg!"));
    } else if let variable::VariableType::Number(num) = args[0].typed {
        thread::sleep(Duration::new(0, num.try_into().unwrap()));
        return AstResult::Ok(variable::Variable::none_value());
    } else {
        return AstResult::Err(RuntimeError::create_error("sleep():Need one number!"));
    }
}

fn temp_bool(val: bool) -> variable::Variable {
    variable::Variable {
        typed: variable::VariableType::Boolean(val),
        name: Arc::from("# Temp Boolean #"),
    }
}

// Detect if host OS is Unix-like.
pub fn is_unix(
    _: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    AstResult::Ok(temp_bool(cfg!(unix)))
}

// Detect if host OS is Microsoft Windows.
pub fn is_win(
    _: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    AstResult::Ok(temp_bool(cfg!(windows)))
}

// Detect if host OS is Linux.
pub fn is_linux(
    _: Vec<variable::Variable>,
    _: &mut Context,
) -> AstResult {
    AstResult::Ok(temp_bool(cfg!(target_os = "linux")))
}
