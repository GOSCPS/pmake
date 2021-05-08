//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    help.rs
// Content: pmake standard help function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, error, variable};
use crate::standard::help::error::RuntimeError;

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
