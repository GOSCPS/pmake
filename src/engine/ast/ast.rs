//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    ast.rs
// Content: pmake ast source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{error, variable};

// Ast定义
pub trait Ast {
    fn execute(&mut self) -> Result<variable::Variable, error::RuntimeError>;
}
