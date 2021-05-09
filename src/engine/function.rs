//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    function.rs
// Content: pmake function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, variable};
use crate::engine::ast::ast::AstResult;

// 函数
pub type Function = fn(
    args: Vec<variable::Variable>,
    context: &mut Context,
) -> AstResult;
