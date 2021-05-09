//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    variable.rs
// Content: pmake variable source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use std::sync::Arc;

// 变量类型
#[derive(Debug, Clone, std::cmp::PartialEq)]
pub enum VariableType {
    None,
    Str(String),
    Number(i64),
    Boolean(bool),
}

// 变量
#[derive(Debug, Clone)]
pub struct Variable {
    pub typed: VariableType,
    pub name: Arc<str>,
}

impl Variable {
    pub fn none_value() -> Variable {
        Variable {
            typed: VariableType::None,
            name: Arc::from("# TemporaryValue - NONE#"),
        }
    }

    pub fn to_string(&self) -> String {
        return match &self.typed {
            VariableType::None => "None".to_string(),

            VariableType::Str(strs) => strs.to_string(),

            VariableType::Number(num) => num.to_string(),

            VariableType::Boolean(boolean) => boolean.to_string(),
        };
    }
}
