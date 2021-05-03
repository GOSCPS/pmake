//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    function.rs
// Content: pmake function source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::{error, variable};

// 函数
pub trait Function {
    fn execute(
        &mut self,
        args: &Vec<variable::Variable>,
    ) -> Result<variable::Variable, error::RuntimeError>;
}
