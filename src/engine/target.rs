//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    target.rs
// Content: pmake target source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::Ast;

// target
pub struct Target {
    pub name: String,
    pub depends: Vec<String>,
    pub body: Box<dyn Ast>,
}
