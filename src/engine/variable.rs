//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    variable.rs
// Content: pmake variable source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::sync::Arc;

// 变量类型
#[derive(Debug)]
pub enum VariableType {
    None,
    Str(String),
    Number(i64),
    Boolean(bool),
}

// 变量
#[derive(Debug)]
pub struct Variable {
    pub typed: VariableType,
    pub name: Arc<str>,
}

impl Variable {
    fn to_string(&self) -> String {
        return match &self.typed {
            VariableType::None => String::from(" "),
            VariableType::Str(value) => value.to_string(),
            VariableType::Number(num) => num.to_string(),
            VariableType::Boolean(boolean) => boolean.to_string(),
        };
    }
}
