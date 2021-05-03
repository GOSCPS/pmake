//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    rule.rs
// Content: pmake rule source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::Ast;

// 规则
pub struct Rule {
    pub name: String,
    pub import: Vec<String>,
    pub body: Box<dyn Ast>,
}
