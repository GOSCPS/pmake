//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    error.rs
// Content: pmake error source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::tool;
use crate::{engine::ast::ast::Ast, parser::parse::Token};
use std::{error::Error, sync::Arc};

//解析错误
#[derive(Debug)]
pub struct RuntimeError {
    pub reason_token: Option<Token>,
    pub reason_err: Option<Box<dyn Error>>,
    pub reason_str: Option<String>,
    pub help_str: Option<String>,
}

// 实现Display的trait
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(some) = &self.reason_token {
            tool::printer::error_line(&format!(
                "At File {:?} Lines {} Offset {}",
                &some.file, &some.line_number, &some.offset
            ));
            tool::printer::error_line(&format!("{:?}", &some.typed));
        }

        if let Some(some) = &self.reason_str {
            tool::printer::error_line(some);
        }

        if let Some(some) = &self.help_str {
            tool::printer::help_line(some);
        }

        Ok(())
    }
}

impl std::error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reason_err.as_deref()
    }
}
