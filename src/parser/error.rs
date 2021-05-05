//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    error.rs
// Content: pmake parser error source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::{parser::parse::Token, tool};
use std::path::PathBuf;
use std::{error::Error, sync::Arc};

//解析错误
#[derive(Debug)]
pub struct ParseError {
    pub file: Arc<PathBuf>,
    pub line_number: usize,
    pub offset: usize,
    pub length: usize,
    pub source: String,
    pub reason_err: Option<Box<dyn Error>>,
    pub reason_str: Option<String>,
    pub help_str: Option<String>,
    pub reason_token: Option<Token>,
}

// 实现Display的trait
impl std::fmt::Display for ParseError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        tool::printer::error_line(&format!(
            "At File {:?} Lines {} Offset {}:",
            &self.file, self.line_number, self.offset
        ));

        tool::printer::error_line(&self.source);

        // tool::printer::error_line(&format!("{}{}"," ".repeat(self.offset),"^".repeat(self.length)));

        if let Some(some) = &self.reason_str {
            tool::printer::error_line(some);
        }

        if let Some(some) = &self.help_str {
            tool::printer::help_line(some);
        }

        if let Some(some) = &self.reason_token {
            tool::printer::error_line(&format!("From token:{:?}", some));
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reason_err.as_deref()
    }
}
