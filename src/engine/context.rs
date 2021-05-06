//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    context.rs
// Content: pmake context source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{function::Function, variable::Variable};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;

// 上下文
pub struct Context {
    pub variable_table: RwLock<HashMap<String, Variable>>,
}

impl Context {
    pub fn new() -> Context {
        return Context {
            variable_table: RwLock::from(HashMap::new()),
        };
    }
}

// 全局上下文定义
lazy_static! {
    // 全局上下文
    pub static ref GLOBAL_CONTEXT
    : Context = Context::new();

    // 全局函数定义
    pub static ref GLOBAL_FUNCTION
    : Mutex<HashMap<String,Function>>
    = Mutex::from(HashMap::new());
}
