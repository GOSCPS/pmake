//=========================================================
// 这个文件来自 GOSCPS(https://github.com/GOSCPS)
// 使用 GOSCPS 许可证
// File:    error.rs
// Content: pmake error source file
// Copyright (c) 2020-2021 GOSCPS 保留所有权利.
//=========================================================

use crate::engine::ast::ast::Ast;
use crate::parser::parse::Token;
use std::error::Error;
use std::thread;
use std::fmt;

//解析错误
// #[derive(Debug)]
pub struct RuntimeError {
    pub reason_token: Option<Token>,
    pub reason_err: Option<Box<dyn Error + Send + Sync>>,
    pub reason_str: Option<String>,
    pub help_str: Option<String>,
    pub error_ast: Option<Box<dyn Ast>>,
}

// 实现Display的trait
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(some) = &self.reason_token {
            write!(f,"{}:{:?}",thread::current().name().unwrap_or("UNKNOWN"),some)?;
        }

        if let Some(some) = &self.reason_str {
            write!(f,"{}:{}",thread::current().name().unwrap_or("UNKNOWN"),some)?;
        }

        if let Some(some) = &self.help_str {
            write!(f,"{}:{}",thread::current().name().unwrap_or("UNKNOWN"),
            some)?;
        }

        if let Some(some) = &self.error_ast {
            if let Some(pos) = (*some).get_position().clone() {
                write!(f,"{}:At `{:?}` Lines {}",
                thread::current().name().unwrap_or("UNKNOWN"),
                pos.0, pos.1)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.reason_err.as_deref().map(|x| {
            let err: &(dyn Error + 'static) = x;
            err
        })
    }
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeError")
            .field("reason_token", &self.reason_token)
            .field("reason_err", &self.reason_err)
            .field("reason_str", &self.reason_str)
            .field("help_str", &self.help_str)
            .finish()
    }
}

impl RuntimeError {
    // 输出错误到stdout/stderr
    pub fn show_to_console(&self){
        if let Some(some) = &self.reason_token {
            crate::tool::printer::error_line(&format!("{}:{:?}",thread::current().name().unwrap_or("UNKNOWN"),some));
        }

        if let Some(some) = &self.reason_str {
            crate::tool::printer::error_line(&format!("{}:{}",thread::current().name().unwrap_or("UNKNOWN"),some));
        }

        if let Some(some) = &self.help_str {
            crate::tool::printer::help_line(&format!("{}:{}",thread::current().name().unwrap_or("UNKNOWN"),some));
        }

        if let Some(some) = &self.error_ast {
            if let Some(pos) = (*some).get_position().clone() {
                crate::tool::printer::error_line(&format!("{}:At `{:?}` Lines {}",
                thread::current().name().unwrap_or("UNKNOWN"),
                pos.0, pos.1));
            }
        }
    }

    pub fn create_error(err : &str) -> RuntimeError{
        RuntimeError {
            reason_token: None,
            reason_err: None,
            reason_str: Some(err.to_string()),
            help_str: None,
            error_ast: None,
        }
    }
}