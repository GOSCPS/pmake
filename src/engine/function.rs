//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    function.rs
// Content: pmake function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{context::Context, error, variable};

// 函数
pub trait Function: Send + Sync {
    fn execute(
        &mut self,
        args: &Vec<variable::Variable>,
        context: &mut Context,
    ) -> Result<variable::Variable, error::RuntimeError>;
}
